   
import { createContext, Dispatch, ReactNode, SetStateAction, useContext, useState } from "react";

interface QuillPrintersContextType {
    quillPrinters: string | null;
    setQuillPrinters: Dispatch<SetStateAction<string | null>>;
}

const QuillPrintersContext = createContext<QuillPrintersContextType | undefined>(undefined);

export function QuillPrintersProvider({ children }: { children: ReactNode }) {
    const [quillPrinters, setQuillPrinters] = useState<string | null>(null);

    return (
        <QuillPrintersContext.Provider value={{ quillPrinters, setQuillPrinters }}>
            {children}
        </QuillPrintersContext.Provider>
    );
}

export function useQuillPrinters(): QuillPrintersContextType {
    const context = useContext(QuillPrintersContext);
    if (!context) {
        throw new Error("useQuillPrinters must be used within a QuillPrintersProvider");
    }
    return context;
}