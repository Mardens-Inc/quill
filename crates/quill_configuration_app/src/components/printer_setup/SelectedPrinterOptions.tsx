import {useQuillSettings} from "../../providers/QuillSettingsProvider.tsx";
import {Button, Radio, RadioGroup, Separator} from "@heroui/react";
import {cn} from "@heroui/styles";
import {ErrorBoundary} from "../../ErrorBoundry.tsx";
import {Icon} from "@iconify-icon/react";

export function SelectedPrinterOptions()
{
    const {selectedPrinter} = useQuillSettings();
    if (!selectedPrinter) return null;
    return (
        <ErrorBoundary>
            <div className={"flex flex-col gap-4 bg-surface rounded-2xl shadow-sm border py-5.5 px-6 mb-16"}>
                <div className={"flex flex-row justify-between w-full"}>
                    <div className={"flex flex-col"}>
                        <p className={"uppercase font-semibold text-xs text-fg-subtle tracking-[0.07em]"}>active printer</p>
                        <p className={"font-semibold tracking-[-0.01em] text-2xl mt-1"}>{selectedPrinter.printer_name}</p>
                        <p className={"flex flex-row gap-2 text-fg-muted font-mono text-sm mt-0.5"}>
                            <span className={"max-w-70 truncate"}>{selectedPrinter.port_name}</span>
                            <span className={"h-0.75 w-0.75 rounded-full bg-ink-2 my-auto"}/>
                            <span className={"shrink-0"}>{selectedPrinter.dpi} dpi</span>
                            <span className={"h-0.75 w-0.75 rounded-full bg-ink-2 my-auto"}/>
                            <span className={"max-w-80 w-50 truncate"}>{selectedPrinter.driver_name}</span>
                        </p>
                    </div>
                    <div className={"flex flex-row items-center justify-center rounded-full px-2 h-7 bg-success-soft text-success-soft-foreground font-bold text-sm-plus"}>
                        <span className={"w-1.5 h-1.5 bg-success rounded-full mx-1"}/>{selectedPrinter.status}
                    </div>
                </div>
                <div className={"flex flex-row"}>
                    <div className={"flex flex-col shrink grow basis-0 min-w-50"}>
                        <p className={"font-semibold text-md-plus"}>Printer DPI <span className={"uppercase h-5 inline-flex items-center px-2 rounded-xl font-semibold text-xxs text-important-fg bg-important-bg tracking-[0.03em]"}>important</span></p>
                        <RadioGroup defaultValue={"auto"} className={"flex-row gap-4"}>
                            <Radio value={"auto"}>
                                <Radio.Content
                                    className={cn(
                                        "text-accent border-transparent rounded-[9px] border h-9.5 text-md",
                                        "font-semibold grow shrink basis-0 px-4 py-5",
                                        "data-selected:bg-selected-printer data-selected:border-accent"
                                    )}
                                >
                                    Auto - {selectedPrinter.dpi} dpi
                                </Radio.Content>
                            </Radio>
                            <Radio value={"manual"}>
                                <Radio.Content
                                    className={cn(
                                        "text-fg-muted border-transparent rounded-[9px] border h-9.5 text-md",
                                        "font-semibold grow shrink basis-0 px-4 py-5",
                                        "data-selected:bg-selected-printer data-selected:border-accent data-selected:text-accent"
                                    )}
                                >
                                    Manual override
                                </Radio.Content>
                            </Radio>
                        </RadioGroup>
                        <p className={"mt-2 leading-[1.45] text-sm-plus text-fg-muted"}>
                            Wrong DPI is the most common cause of<br />stretched or shrunken labels. Leave on Auto<br/>unless the printed size is visibly off.
                        </p>
                    </div>
                    <div className={"flex flex-col min-w-50 shrink grow basis-0"}>
                        <p className={"font-semibold mb-2 text-md-plus"}>Last test print</p>
                        <p className={"h-9.5 px-3.25 rounded-[9px] bg-surface-alt text-fg-muted font-mono text-md items-center flex"}>Never tested</p>
                        <p className={"mt-2 leading-[1.45] text-sm-plus"}>A calibration label prints a ruler, the printer name and current DPI so you can confirm alignment.</p>
                    </div>
                </div>
                <Separator />
                <Button size={"lg"} className={"primary"}>
                    <Icon icon={"solar:play-bold"} />
                    Test print
                </Button>
            </div>
        </ErrorBoundary>
    );
}