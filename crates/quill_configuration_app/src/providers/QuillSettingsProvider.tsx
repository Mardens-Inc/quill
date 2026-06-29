   
import { createContext, Dispatch, ReactNode, SetStateAction, useContext, useState } from "react";

interface QuillSettingsContextType {
    quillSettings: string | null;
    setQuillSettings: Dispatch<SetStateAction<string | null>>;
}

const QuillSettingsContext = createContext<QuillSettingsContextType | undefined>(undefined);

export function QuillSettingsProvider({ children }: { children: ReactNode }) {
    const [quillSettings, setQuillSettings] = useState<string | null>(null);

    return (
        <QuillSettingsContext.Provider value={{ quillSettings, setQuillSettings }}>
            {children}
        </QuillSettingsContext.Provider>
    );
}

export function useQuillSettings(): QuillSettingsContextType {
    const context = useContext(QuillSettingsContext);
    if (!context) {
        throw new Error("useQuillSettings must be used within a QuillSettingsProvider");
    }
    return context;
}