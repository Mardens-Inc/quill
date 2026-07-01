import {createContext, ReactNode, useContext, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

type AboutContextType = {
    version: string,
    buildNumber: string,
    configSchemaVersion: number,
    helperRunning: boolean,
    helperVersion: Version
}

type Version = {
    version: string,
    build: string,
}

const AboutContext = createContext<AboutContextType | undefined>(undefined);

export function AboutProvider({children}: { children: ReactNode })
{
    const [context, setContext] = useState({} as AboutContextType);
    useEffect(() =>
    {
        const refresh = async () =>
        {
            const response: AboutContextType = await invoke("about");
            setContext(response);
        };
        refresh().then();

        const interval = setInterval(refresh, 5000);
        return () => clearInterval(interval);
    }, []);


    return (
        <AboutContext.Provider value={context}>
            {children}
        </AboutContext.Provider>
    );
}

export function useAbout(): AboutContextType
{
    const context = useContext(AboutContext);
    if (!context)
    {
        throw new Error("useAbout must be used within a AboutProvider");
    }
    return context;
}