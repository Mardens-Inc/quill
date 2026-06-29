import {Button} from "@heroui/react";
import {Icon} from "@iconify-icon/react";

export default function PrinterSetupPage()
{
    return (
        <div className={"flex justify-center w-full"}>
            <div className={"flex flex-col gap-2 mx-8 mt-5 w-full max-w-4xl"}>
                <h1 className={"font-bold text-display"}>Printer setup</h1>
                <p className={"font-light text-fg-muted mt-0.75"}>Detect the USB label printer and confirm it prints a clean calibration label.</p>
                <div className={"flex flex-col gap-4 bg-surface py-6 rounded-2xl shadow-sm border"}>
                    <div className={"flex flex-row items-center pb-4 px-6 border-b"}>
                        <div className={"flex flex-col gap-2 w-full"}>
                            <p className={"font-bold text-lg"}>Detected printers</p>
                            <p className={"text-md text-muted mt-0.75"}>Connect a printer over USB, then scan. Operators print to whichever printer you select here.</p>
                        </div>
                        <Button variant={"outline"} className={"text-lg"}><Icon icon={"tabler:reload"}/> Scan for printers</Button>
                    </div>

                    <div className={"flex flex-row items-center pb-4 px-6 justify-center"}>
                        <div className={"flex flex-col gap-2"}>
                            <div className={"w-13.5 h-13.5 shrink-0 text-fg-subtle bg-surface-alt flex items-center justify-center rounded-2xl"}>
                                <Icon width={25} height={25} icon={"lucide:printer"}/>
                            </div>
                            <p>No printers found</p>

                        </div>
                    </div>

                </div>
            </div>
        </div>
    );
}