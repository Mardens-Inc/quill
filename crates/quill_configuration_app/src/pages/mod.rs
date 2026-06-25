#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Pages {
	#[default]
	Printer,
	Stocks,
	PrintSettings,
	Server,
	Logs,
	About,
}