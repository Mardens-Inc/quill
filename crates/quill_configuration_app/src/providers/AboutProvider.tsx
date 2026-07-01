   
import { createContext, Dispatch, ReactNode, SetStateAction, useContext, useState } from "react";

interface AboutContextType {
    about: string | null;
    setAbout: Dispatch<SetStateAction<string | null>>;
}

const AboutContext = createContext<AboutContextType | undefined>(undefined);

export function AboutProvider({ children }: { children: ReactNode }) {
    const [about, setAbout] = useState<string | null>(null);

    return (
        <AboutContext.Provider value={{ about, setAbout }}>
            {children}
        </AboutContext.Provider>
    );
}

export function useAbout(): AboutContextType {
    const context = useContext(AboutContext);
    if (!context) {
        throw new Error("useAbout must be used within a AboutProvider");
    }
    return context;
}