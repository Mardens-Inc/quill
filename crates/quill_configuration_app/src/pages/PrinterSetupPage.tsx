import {PrinterList} from "../components/printer_setup/PrinterList.tsx";
import {SelectedPrinterOptions} from "../components/printer_setup/SelectedPrinterOptions.tsx";

export default function PrinterSetupPage()
{
    return (
        <div className={"flex justify-center w-full"}>
            <div className={"flex flex-col gap-2 mx-8 mt-5 w-full max-w-4xl"}>
                <h1 className={"font-bold text-display tracking-[-0.02em]"}>Printer setup</h1>
                <p className={"font-light text-fg-muted mt-0.75 text-base-plus"}>Detect the USB label printer and confirm it prints a clean calibration label.</p>
                <PrinterList/>
                <SelectedPrinterOptions/>
            </div>
        </div>
    );
}