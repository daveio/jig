#!/usr/bin/env python3
"""
Linear Tickets Management Script

This script reads tickets.yaml and creates/updates tickets in Linear using the GraphQL API.
It supports idempotent execution - safe to run multiple times.

Requirements:
- LINEAR_API_KEY environment variable
- tickets.yaml file in the same directory
- Python packages: requests, pyyaml, python-dotenv

Usage:
    python tickets.py [--dry-run] [--verbose]
"""

import os
import sys
import json
import time
import hashlib
import argparse
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass
from pathlib import Path

try:
    import yaml
    import requests
    from dotenv import load_dotenv
except ImportError as e:
    print(f"Missing required package: {e}")
    print("Install with: pip install requests pyyaml python-dotenv")
    sys.exit(1)

# Load environment variables
load_dotenv()

@dataclass
class LinearTeam:
    id: str
    name: str
    key: str

@dataclass
class LinearLabel:
    id: str
    name: str

@dataclass
class LinearIssue:
    id: str
    identifier: str
    title: str
    description: str
    estimate: Optional[int] = None
    priority: int = 0

# Linear priority mapping
PRIORITY_MAP = {
    1: 2,  # High priority (our 1 -> Linear's 2)
    2: 3,  # Medium priority (our 2 -> Linear's 3)
    3: 4,  # Low priority (our 3 -> Linear's 4)
    0: 0,  # No priority
}

class LinearAPI:
    """Linear GraphQL API client"""

    def __init__(self, api_key: str):
        self.api_key = api_key
        self.base_url = "https://api.linear.app/graphql"
        # Linear API keys (lin_api_*) don't use Bearer prefix
        self.headers = {
            "Authorization": api_key,
            "Content-Type": "application/json",
        }
        self.session = requests.Session()
        self.session.headers.update(self.headers)

    def execute_query(self, query: str, variables: Optional[Dict] = None) -> Dict[str, Any]:
        """Execute a GraphQL query"""
        payload = {
            "query": query,
            "variables": variables or {}
        }

        try:
            return self.handle_graphql_query(payload)
        except requests.exceptions.RequestException as e:
            if hasattr(e, 'response') and e.response is not None:
                try:
                    error_body = e.response.json()
                    if "errors" in error_body:
                        raise Exception(f"API error: {error_body['errors']}")
                except:
                    pass
            raise Exception(f"API request failed: {e}")

    def handle_graphql_query(self, payload):
        response = self.session.post(self.base_url, json=payload, timeout=30)

            # Try to get JSON response even on error
        try:
            result = response.json()
        except json.JSONDecodeError:
            response.raise_for_status()
            raise Exception("Invalid JSON response from API")

        # Check for GraphQL errors
        if "errors" in result:
            error_messages = []
            for error in result["errors"]:
                msg = error.get("message", "Unknown error")
                if "extensions" in error:
                    msg += f" ({error['extensions']})"
                error_messages.append(msg)
            raise Exception(f"GraphQL errors: {'; '.join(error_messages)}")

        # Check HTTP status
        response.raise_for_status()

        if "data" not in result:
            raise Exception(f"No data in response: {result}")

        return result["data"]

    def get_viewer(self) -> Dict[str, Any]:
        """Get current user information"""
        query = """
        query {
            viewer {
                id
                name
                email
            }
        }
        """
        return self.execute_query(query)["viewer"]

    def get_teams(self) -> List[LinearTeam]:
        """Get all teams the user has access to"""
        query = """
        query {
            teams {
                nodes {
                    id
                    name
                    key
                }
            }
        }
        """
        result = self.execute_query(query)
        return [
            LinearTeam(id=team["id"], name=team["name"], key=team["key"])
            for team in result["teams"]["nodes"]
        ]

    def get_team_by_key(self, key: str) -> Optional[LinearTeam]:
        """Get team by key"""
        teams = self.get_teams()
        return next((team for team in teams if team.key.lower() == key.lower()), None)

    def get_labels(self, team_id: str) -> List[LinearLabel]:
        """Get labels for a team"""
        query = """
        query($teamId: String!) {
            team(id: $teamId) {
                labels {
                    nodes {
                        id
                        name
                        color
                    }
                }
            }
        }
        """
        result = self.execute_query(query, {"teamId": team_id})
        if not result["team"]:
            return []
        return [
            LinearLabel(id=label["id"], name=label["name"])
            for label in result["team"]["labels"]["nodes"]
        ]

    def create_label(self, team_id: str, name: str, description: str = "", color: str = "#5E6AD2") -> str:
        """Create a new label"""
        mutation = """
        mutation($teamId: String!, $name: String!, $description: String, $color: String!) {
            issueLabelCreate(input: {
                teamId: $teamId
                name: $name
                description: $description
                color: $color
            }) {
                issueLabel {
                    id
                    name
                }
                success
            }
        }
        """
        variables = {
            "teamId": team_id,
            "name": name,
            "description": description,
            "color": color
        }
        result = self.execute_query(mutation, variables)
        if not result["issueLabelCreate"]["success"]:
            raise Exception(f"Failed to create label: {name}")
        return result["issueLabelCreate"]["issueLabel"]["id"]

    def get_issues(self, team_id: str) -> List[LinearIssue]:
        """Get all issues for a team (first 250)"""
        query = """
        query($teamId: String!) {
            team(id: $teamId) {
                issues(first: 250) {
                    nodes {
                        id
                        identifier
                        title
                        description
                        estimate
                        priority
                    }
                }
            }
        }
        """
        result = self.execute_query(query, {"teamId": team_id})
        if not result["team"]:
            return []
        return [
            LinearIssue(
                id=issue["id"],
                identifier=issue["identifier"],
                title=issue["title"],
                description=issue["description"] or "",
                estimate=issue["estimate"],
                priority=issue["priority"]
            )
            for issue in result["team"]["issues"]["nodes"]
        ]

    def create_issue(self, team_id: str, title: str, description: str = "",
                    parent_id: Optional[str] = None, estimate: Optional[int] = None,
                    priority: int = 0, label_ids: List[str] = None) -> str:
        """Create a new issue"""
        mutation = """
        mutation($teamId: String!, $title: String!, $description: String,
                $parentId: String, $estimate: Float, $priority: Float, $labelIds: [String!]) {
            issueCreate(input: {
                teamId: $teamId
                title: $title
                description: $description
                parentId: $parentId
                estimate: $estimate
                priority: $priority
                labelIds: $labelIds
            }) {
                issue {
                    id
                    identifier
                    title
                }
                success
            }
        }
        """
        variables = {
            "teamId": team_id,
            "title": title,
            "description": description,
            "parentId": parent_id,
            "estimate": float(estimate) if estimate else None,
            "priority": float(priority),
            "labelIds": label_ids or []
        }
        result = self.execute_query(mutation, variables)
        if not result["issueCreate"]["success"]:
            raise Exception(f"Failed to create issue: {title}")
        return result["issueCreate"]["issue"]["id"]

    def update_issue(self, issue_id: str, title: Optional[str] = None,
                    description: Optional[str] = None, estimate: Optional[int] = None,
                    priority: Optional[int] = None, label_ids: Optional[List[str]] = None) -> bool:
        """Update an existing issue"""
        mutation = """
        mutation($issueId: String!, $title: String, $description: String,
                $estimate: Float, $priority: Float, $labelIds: [String!]) {
            issueUpdate(id: $issueId, input: {
                title: $title
                description: $description
                estimate: $estimate
                priority: $priority
                labelIds: $labelIds
            }) {
                issue {
                    id
                }
                success
            }
        }
        """
        # Build input only with non-None values
        input_data = {}
        if title is not None:
            input_data["title"] = title
        if description is not None:
            input_data["description"] = description
        if estimate is not None:
            input_data["estimate"] = float(estimate)
        if priority is not None:
            input_data["priority"] = float(priority)
        if label_ids is not None:
            input_data["labelIds"] = label_ids

        variables = {
            "issueId": issue_id,
            **input_data
        }

        result = self.execute_query(mutation, variables)
        return result["issueUpdate"]["success"]

class TicketsManager:
    """Manages the creation and updating of Linear tickets"""

    def __init__(self, api: LinearAPI, dry_run: bool = False, verbose: bool = False):
        self.api = api
        self.dry_run = dry_run
        self.verbose = verbose
        self.team: Optional[LinearTeam] = None
        self.labels: Dict[str, str] = {}  # name -> id mapping
        self.existing_issues: Dict[str, LinearIssue] = {}  # title -> issue mapping
        self.created_issues: Dict[str, str] = {}  # internal_id -> linear_id mapping

    def log(self, message: str, level: str = "INFO"):
        """Log a message"""
        if level == "DEBUG" and not self.verbose:
            return
        prefix = "[DRY RUN] " if self.dry_run else ""
        print(f"{prefix}[{level}] {message}")

    def setup_team(self, team_key: str):
        """Set up team and get existing data"""
        self.log(f"Setting up team: {team_key}")

        # Get team
        self.team = self.api.get_team_by_key(team_key)
        if not self.team:
            raise Exception(f"Team '{team_key}' not found")

        self.log(f"Found team: {self.team.name} ({self.team.key})")

        # Get existing labels
        existing_labels = self.api.get_labels(self.team.id)
        self.labels = {label.name: label.id for label in existing_labels}
        self.log(f"Found {len(self.labels)} existing labels")

        # Get existing issues
        # Note: This only gets first 250 issues. For teams with more issues,
        # we'd need to implement pagination with hasNextPage/endCursor
        existing_issues = self.api.get_issues(self.team.id)
        self.existing_issues = {issue.title: issue for issue in existing_issues}
        self.log(f"Found {len(self.existing_issues)} existing issues")

    def ensure_labels(self, label_configs: List[Dict[str, Any]]):
        """Ensure all required labels exist"""
        self.log("Ensuring labels exist...")

        for label_config in label_configs:
            name = label_config["name"]
            if name not in self.labels:
                self.log(f"Creating label: {name}")
                if not self.dry_run:
                    label_id = self.api.create_label(
                        team_id=self.team.id,
                        name=name,
                        description=label_config.get("description", ""),
                        color=label_config.get("color", "#5E6AD2")
                    )
                    self.labels[name] = label_id
                    time.sleep(2)  # Rate limiting
            else:
                self.log(f"Label already exists: {name}", "DEBUG")

    def create_or_update_issue(self, issue_config: Dict[str, Any],
                              parent_id: Optional[str] = None) -> str:
        """Create or update a single issue"""
        title = issue_config["title"]
        description = issue_config["description"]
        estimate = issue_config.get("estimate")
        priority_raw = issue_config.get("priority", 0)
        # Map our priority values to Linear's format
        priority = PRIORITY_MAP.get(priority_raw, 0)
        label_names = issue_config.get("labels", [])

        # Convert label names to IDs
        label_ids = [self.labels.get(name) for name in label_names if name in self.labels]
        label_ids = [lid for lid in label_ids if lid]  # Remove None values

        # Check if issue exists
        existing = self.existing_issues.get(title)

        if existing:
            self.log(f"Updating existing issue: {title}")
            if not self.dry_run:
                success = self.api.update_issue(
                    issue_id=existing.id,
                    title=title,
                    description=description,
                    estimate=estimate,
                    priority=priority,
                    label_ids=label_ids or None
                )
                if not success:
                    raise Exception(f"Failed to update issue: {title}")
                time.sleep(2)  # Rate limiting
            return existing.id
        else:
            self.log(f"Creating new issue: {title}")
            if not self.dry_run:
                issue_id = self.api.create_issue(
                    team_id=self.team.id,
                    title=title,
                    description=description,
                    parent_id=parent_id,
                    estimate=estimate,
                    priority=priority,
                    label_ids=label_ids
                )
                # Add to existing issues for future reference
                self.existing_issues[title] = LinearIssue(
                    id=issue_id,
                    identifier="",  # Will be assigned by Linear
                    title=title,
                    description=description,
                    estimate=estimate,
                    priority=priority
                )
                time.sleep(2)  # Rate limiting
                return issue_id
            else:
                return f"dry-run-{abs(hash(title))}"

    def process_tickets(self, tickets_data: Dict[str, Any]):
        """Process all tickets from the YAML data"""
        metadata = tickets_data.get("metadata", {})
        team_key = metadata.get("team", "jig")

        # Setup team
        self.setup_team(team_key)

        # Ensure labels exist
        labels_config = metadata.get("labels", [])
        self.ensure_labels(labels_config)

        # Process epics
        epics = tickets_data.get("epics", [])
        self.log(f"Processing {len(epics)} epics...")

        for epic in epics:
            epic_id = self.create_or_update_issue(epic)
            self.created_issues[epic["id"]] = epic_id
            self.log(f"Epic processed: {epic['title']}")

        # Process features
        features = tickets_data.get("features", [])
        self.log(f"Processing {len(features)} features...")

        for feature in features:
            parent_epic_id = self.created_issues.get(feature.get("epic_id"))
            feature_id = self.create_or_update_issue(feature, parent_epic_id)
            self.created_issues[feature["id"]] = feature_id
            self.log(f"Feature processed: {feature['title']}")

        # Process tasks
        tasks = tickets_data.get("tasks", [])
        self.log(f"Processing {len(tasks)} tasks...")

        for task in tasks:
            parent_feature_id = self.created_issues.get(task.get("feature_id"))
            task_id = self.create_or_update_issue(task, parent_feature_id)
            self.created_issues[task["id"]] = task_id
            self.log(f"Task processed: {task['title']}")

        self.log("All tickets processed successfully!")

def load_tickets_yaml() -> Dict[str, Any]:
    """Load and parse the tickets.yaml file"""
    yaml_path = Path(__file__).parent / "tickets.yaml"

    if not yaml_path.exists():
        raise FileNotFoundError(f"tickets.yaml not found at {yaml_path}")

    try:
        with open(yaml_path, 'r', encoding='utf-8') as f:
            return yaml.safe_load(f)
    except yaml.YAMLError as e:
        raise Exception(f"Failed to parse tickets.yaml: {e}")

def main():
    parser = argparse.ArgumentParser(description="Manage Linear tickets from YAML configuration")
    parser.add_argument("--dry-run", action="store_true", help="Show what would be done without making changes")
    parser.add_argument("--verbose", "-v", action="store_true", help="Enable verbose logging")
    args = parser.parse_args()

    # Get API key
    api_key = os.getenv("LINEAR_API_KEY")
    if not api_key:
        print("ERROR: LINEAR_API_KEY environment variable not set")
        print("Get your API key from: https://linear.app/settings/api")
        sys.exit(1)

    # Validate API key format
    if not api_key.startswith("lin_api_"):
        print("WARNING: API key should start with 'lin_api_'")
        print("Make sure you're using a personal API key from Linear settings")

    try:
        process(api_key, args)
    except KeyboardInterrupt:
        print("\nOperation cancelled by user")
        sys.exit(1)
    except Exception as e:
        print(f"ERROR: {e}")
        if args.verbose:
            import traceback
            traceback.print_exc()
        sys.exit(1)


def process(api_key, args):
    # Initialize API client
    api = LinearAPI(api_key)

    # Test connection
    viewer = api.get_viewer()
    print(f"Connected to Linear as: {viewer['name']} ({viewer['email']})")

    # Load tickets configuration
    tickets_data = load_tickets_yaml()
    print(f"Loaded tickets configuration: {len(tickets_data.get('epics', []))} epics, "
          f"{len(tickets_data.get('features', []))} features, {len(tickets_data.get('tasks', []))} tasks")

    # Process tickets
    manager = TicketsManager(api, dry_run=args.dry_run, verbose=args.verbose)
    manager.process_tickets(tickets_data)

    if args.dry_run:
        print("\nDry run completed. No changes were made.")
    else:
        print("\nTickets processed successfully!")

if __name__ == "__main__":
    main()
