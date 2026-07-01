import {ErrorBoundary} from "../../ErrorBoundry.tsx";
import {Button, Radio, RadioGroup, Spinner} from "@heroui/react";
import {PrinterStatus, useQuillPrinters} from "../../providers/QuillPrintersProvider.tsx";
import {useQuillSettings} from "../../providers/QuillSettingsProvider.tsx";
import {Icon} from "@iconify-icon/react";

export function PrinterList()
{
    const {isRefreshing, refresh, printers} = useQuillPrinters();
    const {setSelectedPrinter, settings} = useQuillSettings();

    return (
        <ErrorBoundary>
            <div className={"flex flex-col gap-4 bg-surface py-6 rounded-2xl shadow-sm border"}>
                <div className={"flex flex-row items-center pb-4 px-6 border-b"}>
                    <div className={"flex flex-col gap-2 w-full"}>
                        <p className={"font-bold text-lg"}>Detected printers</p>
                        <p className={"text-md text-muted mt-0.75"}>Connect a printer over USB, then scan. Operators print to whichever printer you select here.</p>
                    </div>
                    <Button variant={"outline"} className={"text-lg"} isPending={isRefreshing} onPress={refresh}>{isRefreshing ? <Spinner size={"sm"}/> : <Icon icon={"tabler:reload"}/>} Scan for printers</Button>
                </div>

                {printers.length == 0 ?

                    <div className={"flex flex-row items-center pb-4 px-6 justify-center"}>
                        <div className={"flex flex-col gap-2 items-center p-10"}>
                            <div className={"w-13.5 h-13.5 shrink-0 text-fg-subtle bg-surface-alt flex items-center justify-center rounded-2xl"}>
                                <Icon width={25} height={25} icon={"lucide:printer"}/>
                            </div>
                            <p className={"font-bold text-lg mt-2"}>No printers found</p>
                            <p className={"text-center text-md text-fg-muted"}>Make sure the label printer is powered on and connected <br/>
                                by USB, then click <b className={"text-fg"}>Scan for printers.</b></p>

                        </div>
                    </div>
                    :
                    <div className={"max-h-100 overflow-auto"}>
                        <RadioGroup value={settings.selectedPrinter} onChange={setSelectedPrinter}>
                            {printers.map(printer => (
                                <Radio value={printer.printer_name} className={"mt-0"}>
                                    <Radio.Content className={"data-selected:bg-selected-printer w-full px-6 h-18.25"}>
                                        <Radio.Control>
                                            <Radio.Indicator/>
                                        </Radio.Control>
                                        <div className={"flex flex-row justify-between w-full"}>
                                            <div className={"flex flex-col"}>
                                                <p className={"font-semibold text-base"}>{printer.printer_name}</p>
                                                <p className={"flex flex-row gap-2 text-fg-muted font-mono text-sm mt-0.5"}>
                                                    <span className={"max-w-25 truncate"}>{printer.port_name}</span>
                                                    <span className={"h-0.75 w-0.75 rounded-full bg-ink-2 my-auto"}/>
                                                    <span className={"shrink-0"}>{printer.dpi} dpi</span>
                                                    <span className={"h-0.75 w-0.75 rounded-full bg-ink-2 my-auto"}/>
                                                    <span className={"max-w-50 w-50 truncate"}>{printer.driver_name}</span>
                                                </p>
                                            </div>
                                            {
                                                printer.status == PrinterStatus.Idle ?
                                                    <div className={"flex flex-row items-center justify-center rounded-full relative px-2.75 h-6.5 bg-success/20 text-success-soft-foreground font-semibold text-sm"}>
                                                        <span className={"w-1.5 h-1.5 bg-success rounded-full mr-1"}/>Connected
                                                    </div>
                                                    :
                                                    printer.status == PrinterStatus.Error ?
                                                        <div className={"flex flex-row items-center justify-center rounded-full relative px-2.75 h-6.5 bg-danger/20 text-danger-soft-foreground font-semibold text-sm"}>
                                                            <span className={"w-1.5 h-1.5 bg-danger rounded-full mr-1"}/>Error
                                                        </div>
                                                        :
                                                        <div className={"flex flex-row items-center justify-center rounded-full relative px-2.75 h-6.5 bg-ink-2/20 text-ink-2 font-semibold text-sm"}>
                                                            <span className={"w-1.5 h-1.5 bg-ink-2 rounded-full mr-1"}/>{printer.status}
                                                        </div>
                                            }
                                        </div>
                                    </Radio.Content>
                                </Radio>
                            ))}
                        </RadioGroup>
                    </div>
                }
            </div>
        </ErrorBoundary>

    );
}