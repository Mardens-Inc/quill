import {Button, ButtonGroup} from "@heroui/react";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {Icon} from "@iconify-icon/react";
import {useEffect, useState} from "react";
import {useQuillSettings} from "../providers/QuillSettingsProvider.tsx";
import iconLight from "../../../../res/icons/quill-light.svg";
import iconDark from "../../../../res/icons/quill-dark.svg";
import {PrinterInfo, useQuillPrinters} from "../providers/QuillPrintersProvider.tsx";
import {useAbout} from "../providers/AboutProvider.tsx";

export default function WindowChrome()
{
    const appWindow = getCurrentWindow();
    const {helperRunning} = useAbout();
    const {settings, setDarkMode} = useQuillSettings();
    const {printers} = useQuillPrinters();
    const [selectedPrinter, setSelectedPrinter] = useState<PrinterInfo | undefined>(undefined);

    useEffect(() =>
    {
        appWindow.setDecorations(false).then();
    }, []);
    useEffect(() =>
    {
        setSelectedPrinter(printers.find(i => i.printer_name == settings.selectedPrinter));
    }, [printers]);
    return (

        <div className={"flex flex-row h-12 backdrop-blur-sm sticky top-0 w-full z-51 backdrop-saturate-150 select-none bg-topbar text-ink-0 border-b"}>
            <span className={"absolute h-full w-full"} data-tauri-drag-region=""/>
            <div className={"flex flex-row my-auto ml-4 items-center gap-2"}>
                {settings.darkMode ?
                    <img src={iconLight} alt={"logo"} className={"h-6 w-6"}/>
                    : <img src={iconDark} alt={"logo"} className={"h-6 w-6"}/>
                }
                <p className={"text-lg font-bold select-none uppercase"}>Quill</p>
                <div className={"flex flex-row ml-16 items-center gap-2"}>
                    {helperRunning ?
                        <div className={"flex flex-row items-center justify-center rounded-full px-2 py-1 bg-success-soft text-success-soft-foreground font-bold text-sm-plus"}>
                            <span className={"w-1.5 h-1.5 bg-success rounded-full mx-1"}/>Helper Running
                        </div>
                        : <div className={"flex flex-row items-center justify-center rounded-full px-2 py-1 bg-danger-soft text-danger-soft-foreground font-bold text-sm-plus"}>
                            <span className={"w-1.5 h-1.5 bg-danger rounded-full mx-1"}/>Helper Offline
                        </div>
                    }
                    <div className={"flex flex-row items-center justify-center rounded-full px-2 py-1 bg-slate-200 dark:bg-surface-2 text-ink-2 font-semibold text-sm-plus"}>
                        <span className={"w-1.5 h-1.5 bg-ink-2 rounded-full mx-1"}/> {selectedPrinter?.status ?? "Unknown"}
                    </div>
                    <div className={"flex flex-row items-center justify-center rounded-full px-2 py-1 bg-transparent border font-mono text-ink-2 font-normal text-sm-plus "}>
                        <Icon icon={"lucide:printer"} className={"mx-1"}/>
                        <span className={"max-w-37.5 truncate"}>{settings.selectedPrinter ? settings.selectedPrinter : "No printer selected"}</span>
                    </div>
                </div>
            </div>
            <div className={"flex flex-row ml-auto h-full gap-2"}>
                {settings.darkMode ?
                    <Button onPress={() => setDarkMode(false)} className={"my-auto"} variant={"outline"}>
                        <Icon icon={"lucide:sun"}/>
                        Light
                    </Button>
                    :
                    <Button onPress={() => setDarkMode(true)} className={"my-auto"} variant={"outline"}>
                        <Icon icon={"lucide:moon"}/>
                        Dark
                    </Button>
                }
                <ButtonGroup className={"h-full"} variant={"tertiary"}>
                    <Button variant={"ghost"} className={"min-w-0 h-full rounded-none text-[1rem] hover:bg-surface-1"} onPress={() => appWindow.minimize()}><Icon icon="material-symbols:minimize-rounded"/></Button>
                    <Button variant={"ghost"} className={"min-w-0 h-full rounded-none text-[0.75rem] hover:bg-surface-1"} onPress={() => appWindow.toggleMaximize()}><Icon icon="material-symbols:square-outline-rounded"/></Button>
                    <Button variant={"ghost"} className={"min-w-0 h-full rounded-none text-[1rem] hover:bg-danger hover:text-danger-foreground"} onPress={() => appWindow.close()}><Icon icon="material-symbols:close-rounded"/></Button>
                </ButtonGroup>
            </div>
        </div>
    );
}

