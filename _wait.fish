#!/usr/bin/env fish

# Wait for GitHub job 16332911713 to complete
echo "Waiting for GitHub job 16332911713 to complete..."

while true
    set jobstatus (gh run view 16332911713 --json status --jq '.status' | cat)

    if test "$jobstatus" = "completed"
        echo "Job completed!"
        break
    else
        echo "Job status: $jobstatus - waiting..."
        sleep 10
    end
end

# Sleep 20 seconds
echo "Sleeping for 20 seconds..."
sleep 20

# Run the commands
echo "Running git add, oco, and push..."
git add -A .
oco --fgm --yes
push

echo "Done!"
