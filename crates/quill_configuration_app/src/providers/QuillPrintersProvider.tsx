import {createContext, ReactNode, useContext, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

export type PrinterInfo = {
    server_name: string | undefined,
    printer_name: string,
    share_name: string | undefined,
    port_name: string,
    driver_name: string,
    comment: string | undefined,
    location: string | undefined,
    print_processor: string,
    datatype: string,
    parameters: string | undefined,
    sep_file: string | undefined,
    attributes: number,
    priority: number,
    default_priority: number,
    start_time: number,
    until_time: number,
    status: PrinterStatus,
    jobs: number,
    average_ppm: number,
    dpi: number|undefined,
}

export enum PrinterStatus
{
    /// The printer is idle
    Idle = "Idle",
    /// The printer is paused.
    Paused = "Paused",
    /// The printer is in an error state.
    Error = "Error",
    /// The printer is being deleted.
    PendingDeletion = "PendingDeletion",
    /// Paper is jammed in the printer
    PaperJam = "PaperJam",
    /// The printer is out of paper.
    PaperOut = "PaperOut",
    /// The printer is in a manual feed state.
    ManualFeed = "ManualFeed",
    /// The printer has a paper problem.
    PaperProblem = "PaperProblem",
    /// The printer is offline.
    Offline = "Offline",
    /// The printer is in an active input/output state
    IoActive = "IoActive",
    /// The printer is busy.
    Busy = "Busy",
    /// The printer is printing.
    Printing = "Printing",
    /// The printer's output bin is full.
    OutputBinFull = "OutputBinFull",
    /// The printer is not available for printing.
    NotAvailable = "NotAvailable",
    /// The printer is waiting.
    Waiting = "Waiting",
    /// The printer is processing a print job.
    Processing = "Processing",
    /// The printer is initializing.
    Initializing = "Initializing",
    /// The printer is warming up.
    WarmingUp = "WarmingUp",
    /// The printer is low on toner.
    TonerLow = "TonerLow",
    /// The printer is out of toner.
    NoToner = "NoToner",
    /// The printer cannot print the current page.
    PagePunt = "PagePunt",
    /// The printer requires user intervention.
    UserIntervention = "UserIntervention",
    /// The printer has run out of memory.
    OutOfMemory = "OutOfMemory",
    /// The printer door is open.
    DoorOpen = "DoorOpen",
    /// The printer server status is unknown.
    ServerUnknown = "ServerUnknown",
    /// The printer is in power save mode.
    PowerSave = "PowerSave",
    /// The printer server is offline.
    ServerOffline = "ServerOffline",
    /// The printer driver needs to be updated.
    DriverUpdateNeeded = "DriverUpdateNeeded",
}

interface QuillPrintersContextType
{
    printers: PrinterInfo[];
    refresh: () => Promise<void>;
    isRefreshing: boolean;
}

const QuillPrintersContext = createContext<QuillPrintersContextType | undefined>(undefined);

export function QuillPrintersProvider({children}: { children: ReactNode })
{
    const [quillPrinters, setQuillPrinters] = useState<PrinterInfo[]>([]);
    const [isRefreshing, setIsRefreshing] = useState(false);

    useEffect(() =>
    {
        refresh();
        const refresh_interval = setInterval(async () =>
        {
            await refresh();
        }, 30_000);
        return () => clearInterval(refresh_interval);
    }, []);

    const refresh = async () =>
    {
        console.info("Refreshing printers...");
        setIsRefreshing(true);
        const response: PrinterInfo[] = await invoke("list_printers");
        setQuillPrinters(response);
        console.info("Found printers: ", response.map(i => i.printer_name).join(", "));
        console.debug("Loaded printers", response);
        setTimeout(() =>
        {
            setIsRefreshing(false);
        }, 1500); // timeout to prevent spamming
    };

    return (
        <QuillPrintersContext.Provider value={{printers: quillPrinters, refresh, isRefreshing}}>
            {children}
        </QuillPrintersContext.Provider>
    );
}

export function useQuillPrinters(): QuillPrintersContextType
{
    const context = useContext(QuillPrintersContext);
    if (!context)
    {
        throw new Error("useQuillPrinters must be used within a QuillPrintersProvider");
    }
    return context;
}