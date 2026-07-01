   
import { createContext, Dispatch, ReactNode, SetStateAction, useContext, useState } from "react";

interface LabelStockModalContextType {
    labelStockModal: string | null;
    setLabelStockModal: Dispatch<SetStateAction<string | null>>;
}

const LabelStockModalContext = createContext<LabelStockModalContextType | undefined>(undefined);

export function LabelStockModalProvider({ children }: { children: ReactNode }) {
    const [labelStockModal, setLabelStockModal] = useState<string | null>(null);

    return (
        <LabelStockModalContext.Provider value={{ labelStockModal, setLabelStockModal }}>
            {children}
        </LabelStockModalContext.Provider>
    );
}

export function useLabelStockModal(): LabelStockModalContextType {
    const context = useContext(LabelStockModalContext);
    if (!context) {
        throw new Error("useLabelStockModal must be used within a LabelStockModalProvider");
    }
    return context;
}