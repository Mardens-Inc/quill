import {createContext, ReactNode, useCallback, useContext, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {PrinterInfo, useQuillPrinters} from "./QuillPrintersProvider.tsx";

type QuillSettings = {
    darkMode: boolean,
    selectedPrinter: string | undefined,
    labels: LabelStock[],
    helperServicePort: number,
    density: number,
    printIps: number,
    defaultOrientation: number,
    scale: number,
    monochromeThreshold: number,
    allowedOrigins: string[],
}

export type LabelStock = {
    id?: string,
    name: string,
    width: number,
    height: number,
    gap: number,
    linerL: number,
    linerR: number,
}

type QuillSettingsContextType = {
    settings: QuillSettings;
    setDarkMode: (value: boolean) => void;
    setSelectedPrinter: (value: string) => void;
    selectedPrinter: PrinterInfo | undefined;
    setDensity: (value: number) => void;
    setPrintIps: (value: number) => void;
    setDefaultOrientation: (value: number) => void;
    setScale: (value: number) => void;
    setMonochromeThreshold: (value: number) => void;
    setListenPort: (value: number) => void;
    setAllowedOrigins: (value: string[]) => void;
    createLabel: (value: LabelStock) => void;
    editLabel: (value: LabelStock) => void;
    deleteLabel: (id: string) => void;
}

const QuillSettingsContext = createContext<QuillSettingsContextType | undefined>(undefined);

export function QuillSettingsProvider({children}: { children: ReactNode })
{
    const [quillSettings, setQuillSettings] = useState<QuillSettings | undefined>(undefined);
    const {printers} = useQuillPrinters();
    const selectedPrinter = printers.find(i => i.printer_name == quillSettings?.selectedPrinter);
    useEffect(() =>
    {
        load();
    }, []);

    useEffect(() =>
    {
        if (quillSettings == undefined) return;
        if (quillSettings.darkMode) document.documentElement.classList.add("dark");
        else document.documentElement.classList.remove("dark");
        save();
    }, [quillSettings]);

    const load = () =>
    {
        invoke("load").then(value =>
        {
            console.log("Loaded settings", value);
            setQuillSettings(value as QuillSettings);
        });
    };

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

    const setDensity = (value: number): void =>
    {
        setQuillSettings(prev => ({...prev, density: value} as QuillSettings));
    };

    const setPrintIps = (value: number): void =>
    {
        setQuillSettings(prev => ({...prev, printIps: value} as QuillSettings));
    };

    const setDefaultOrientation = (value: number): void =>
    {
        setQuillSettings(prev => ({...prev, defaultOrientation: value} as QuillSettings));
    };

    const setScale = (value: number): void =>
    {
        setQuillSettings(prev => ({...prev, scale: value} as QuillSettings));
    };

    const setMonochromeThreshold = (value: number): void =>
    {
        setQuillSettings(prev => ({...prev, monochromeThreshold: value} as QuillSettings));
    };

    const setAllowedOrigins = (value: string[]): void =>
    {
        setQuillSettings(prev => ({...prev, allowedOrigins: value} as QuillSettings));
    };
    const setListenPort = (value: number): void =>
    {
        setQuillSettings(prev => ({...prev, helperServicePort: value} as QuillSettings));
    };

    const createLabel = (value: LabelStock) =>
    {
        invoke("create_label", {name: value.name, width: value.width, height: value.height, gap: value.gap, linerL: value.linerL, linerR: value.linerR}).then(() => load());
    };
    const deleteLabel = (id: string): void =>
    {
        setQuillSettings(prev => (
            {
                ...prev,
                labels: prev?.labels.filter(i => i.id !== id)
            } as QuillSettings
        ));
    };
    const editLabel = (value: LabelStock) =>
    {
        setQuillSettings(prev => (
            {
                ...prev,
                labels: prev?.labels.map(i =>
                {
                    if (i.id === value.id)
                    {
                        return value;
                    }
                    return i;
                })
            } as QuillSettings
        ));
    };

    return (
        <QuillSettingsContext.Provider value={{settings: quillSettings ?? {} as QuillSettings, setDarkMode, setSelectedPrinter, selectedPrinter, setDensity, setPrintIps, setDefaultOrientation, setScale, setMonochromeThreshold, setAllowedOrigins, createLabel, editLabel, deleteLabel, setListenPort}}>
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