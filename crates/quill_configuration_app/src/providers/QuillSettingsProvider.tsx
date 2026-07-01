import {createContext, ReactNode, useCallback, useContext, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {PrinterInfo, useQuillPrinters} from "./QuillPrintersProvider.tsx";

type QuillSettings = {
    darkMode: boolean,
    selectedPrinter: string | undefined,
    labels: LabelStock[],
    helperServicePort: number,
}

type LabelStock = {
    id: string,
    name: string,
    width: number,
    height: number,
    gap: number,
    liner_l: number,
    liner_r: number,
}

type QuillSettingsContextType = {
    settings: QuillSettings;
    setDarkMode: (value: boolean) => void;
    setSelectedPrinter: (value: string) => void;
    selectedPrinter: PrinterInfo | undefined;
}

const QuillSettingsContext = createContext<QuillSettingsContextType | undefined>(undefined);

export function QuillSettingsProvider({children}: { children: ReactNode })
{
    const [quillSettings, setQuillSettings] = useState<QuillSettings | undefined>(undefined);
    const {printers} = useQuillPrinters();
    const selectedPrinter = printers.find(i => i.printer_name == quillSettings?.selectedPrinter);
    useEffect(() =>
    {
        invoke("load").then(value =>
        {
            console.log("Loaded settings", value);
            setQuillSettings(value as QuillSettings);
        });
    }, []);

    useEffect(() =>
    {
        if (quillSettings == undefined) return;
        if (quillSettings.darkMode) document.documentElement.classList.add("dark");
        else document.documentElement.classList.remove("dark");
        save();
    }, [quillSettings]);

    const save = useCallback(() =>
    {
        console.debug("Save settings", quillSettings);
        invoke("save", {value: quillSettings}).then(() =>
        {
            console.info("Saved settings");
        }).catch((message: string) =>
        {
            console.error("Failed to save settings", message);
        });

    }, [quillSettings]);

    const setDarkMode = (value: boolean): void =>
    {
        setQuillSettings(prev => ({
            ...prev,
            darkMode: value
        } as QuillSettings));
        if (value) document.documentElement.classList.add("dark");
        else document.documentElement.classList.remove("dark");
    };

    const setSelectedPrinter = (value: string): void =>
    {
        setQuillSettings(prev => ({
            ...prev,
            selectedPrinter: value
        } as QuillSettings));
    };

    return (
        <QuillSettingsContext.Provider value={{settings: quillSettings ?? {} as QuillSettings, setDarkMode, setSelectedPrinter, selectedPrinter}}>
            {children}
        </QuillSettingsContext.Provider>
    );
}

export function useQuillSettings(): QuillSettingsContextType
{
    const context = useContext(QuillSettingsContext);
    if (!context)
    {
        throw new Error("useQuillSettings must be used within a QuillSettingsProvider");
    }
    return context;
}