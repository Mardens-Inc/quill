import {createContext, ReactNode, useCallback, useContext, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

type QuillSettings = {
    darkMode: boolean,
}

type QuillSettingsContextType = {
    settings: QuillSettings;
    setDarkMode: (value: boolean) => void;
}

const QuillSettingsContext = createContext<QuillSettingsContextType | undefined>(undefined);

export function QuillSettingsProvider({children}: { children: ReactNode })
{

    const [quillSettings, setQuillSettings] = useState<QuillSettings>({} as QuillSettings);
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
        }));
        if (value) document.documentElement.classList.add("dark");
        else document.documentElement.classList.remove("dark");
    };

    return (
        <QuillSettingsContext.Provider value={{settings: quillSettings, setDarkMode}}>
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