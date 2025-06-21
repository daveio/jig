package ui

import "github.com/charmbracelet/lipgloss"

// Color definitions
var (
	Primary   = lipgloss.Color("#7D56F4")
	Secondary = lipgloss.Color("#F25D94")
	Success   = lipgloss.Color("#04B575")
	Warning   = lipgloss.Color("#F9D71C")
	Error     = lipgloss.Color("#EF476F")
	Info      = lipgloss.Color("#06BCF9")
	Muted     = lipgloss.Color("#7C7C7C")
	Text      = lipgloss.Color("#FAFAFA")
	Border    = lipgloss.Color("#383838")
)

// Base styles
var (
	Base = lipgloss.NewStyle().
		Foreground(Text)

	Bold = Base.Bold(true)

	Heading = lipgloss.NewStyle().
		Bold(true).
		Foreground(Primary).
		MarginBottom(1)

	Subheading = lipgloss.NewStyle().
			Bold(true).
			Foreground(Secondary).
			MarginBottom(1)

	ErrorStyle = lipgloss.NewStyle().
			Foreground(Error).
			Bold(true)

	SuccessStyle = lipgloss.NewStyle().
			Foreground(Success).
			Bold(true)

	WarningStyle = lipgloss.NewStyle().
			Foreground(Warning).
			Bold(true)

	InfoStyle = lipgloss.NewStyle().
			Foreground(Info)

	MutedStyle = lipgloss.NewStyle().
			Foreground(Muted)

	CodeStyle = lipgloss.NewStyle().
			Foreground(Info).
			Background(lipgloss.Color("#1A1A1A")).
			Padding(0, 1)

	BorderStyle = lipgloss.NewStyle().
			Border(lipgloss.RoundedBorder()).
			BorderForeground(Border).
			Padding(1, 2)

	ListItemStyle = lipgloss.NewStyle().
			PaddingLeft(2)

	HelpStyle = lipgloss.NewStyle().
			Foreground(Muted).
			Italic(true)
)

// Table styles
var (
	TableHeaderStyle = lipgloss.NewStyle().
				Bold(true).
				Foreground(Primary).
				Align(lipgloss.Center).
				Padding(0, 1)

	TableCellStyle = lipgloss.NewStyle().
			Padding(0, 1).
			Align(lipgloss.Left)

	TableBorderStyle = lipgloss.NewStyle().
				Foreground(Border)
)

// Status indicator styles
var (
	StatusRunning = lipgloss.NewStyle().
			Foreground(Warning).
			SetString("●")

	StatusSuccess = lipgloss.NewStyle().
			Foreground(Success).
			SetString("●")

	StatusError = lipgloss.NewStyle().
			Foreground(Error).
			SetString("●")

	StatusInfo = lipgloss.NewStyle().
			Foreground(Info).
			SetString("●")
)
