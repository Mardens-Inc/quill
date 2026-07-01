import {createContext, ReactNode, useCallback, useContext, useEffect, useState} from "react";
import {Button, Description, Input, Label, Modal, NumberField, TextField} from "@heroui/react";
import {LabelStock, useQuillSettings} from "../../providers/QuillSettingsProvider.tsx";
import {Icon} from "@iconify-icon/react";
import {cn} from "@heroui/styles";


type LabelStockModalProps = {
    isOpen: boolean;
    onClose: () => void;
    labelStock: LabelStock | undefined;
}

function LabelStockModal({isOpen, onClose, labelStock}: LabelStockModalProps)
{
    const {createLabel, editLabel} = useQuillSettings();
    const [name, setName] = useState(labelStock?.name ?? "");
    const [width, setWidth] = useState<number | undefined>(labelStock?.width);
    const [height, setHeight] = useState<number | undefined>(labelStock?.height);
    const [gap, setGap] = useState<number | undefined>(labelStock?.gap ?? 0.12);
    const [linerLeft, setLinerLeft] = useState<number>(labelStock?.linerL ?? 0.05);
    const [linerRight, setLinerRight] = useState<number>(labelStock?.linerR ?? 0.05);
    const isValidFormEntry = name !== "" && width !== undefined && width > 0.4 && height !== undefined && height > 0.4 && gap !== undefined && linerLeft >= 0 && linerRight >= 0;
    const errorMessage: string | undefined = name === "" ? "Enter a preset name" :
        width == undefined || width < 0.4 ? "Width must be a value greater than 0.4mm." :
            height == undefined || height < 0.4 ? "Height must be a value greater than 0.4mm." :
                gap == undefined || gap < 0 ? "Gap must be a positive number." :
                    linerLeft == undefined || linerLeft < 0 ? "Liner Left must be a positive number." :
                        linerRight == undefined || linerRight < 0 ? "Liner Right must be a positive number." :
                            undefined;

    useEffect(() =>
    {
        setName(labelStock?.name ?? "");
        setWidth(labelStock?.width);
        setHeight(labelStock?.height);
        setGap(labelStock?.gap ?? 0.12);
        setLinerLeft(labelStock?.linerL ?? 0.05);
        setLinerRight(labelStock?.linerR ?? 0.05);
    }, [labelStock, isOpen]);


    const save = useCallback(() =>
    {
        if (!isValidFormEntry) return;
        let newLabel: LabelStock = {
            id: undefined,
            name,
            width,
            height,
            gap,
            linerR: linerRight,
            linerL: linerLeft
        };
        if (labelStock?.id)
        {
            newLabel.id = labelStock.id;
            editLabel(newLabel);
        } else
        {
            createLabel(newLabel);
        }

    }, [name, width, height, gap, linerLeft, linerRight, isValidFormEntry, labelStock]);


    return (
        <Modal
            isOpen={isOpen}
            onOpenChange={isOpen =>
            {
                if (!isOpen) onClose();
            }}
        >
            <Modal.Backdrop variant={"blur"}>
                <Modal.Container>
                    <Modal.Dialog>
                        <Modal.Header>
                            <Modal.Heading>{labelStock ? "Edit" : "New"} label stock</Modal.Heading>
                            <Button slot={"close"} isIconOnly variant={"ghost"} size={"lg"}><Icon icon={"lucide:x"}/></Button>
                        </Modal.Header>
                        <Modal.Body className={"flex flex-col gap-4 overflow-y-auto max-h-[calc(80dvh-156px)] px-6 py-5.5"}>
                            <div className={"flex flex-row gap-2"}>
                                <LabelSilhouette width={width ?? 1} height={height ?? 1} className={"w-24 h-24"}/>
                                <TextField value={name} onChange={setName}>
                                    <Label>Preset name</Label>
                                    <Input placeholder={"e.g. 4x6 Shipping"}/>
                                    <Description>Preview scales with width × height below.</Description>
                                </TextField>
                            </div>
                            <div className={"flex flex-row gap-4"}>
                                <NumberField maxValue={4} value={width} onChange={setWidth}>
                                    <Label>Width (mm)</Label>
                                    <NumberField.Input/>
                                </NumberField>
                                <NumberField maxValue={4} value={height} onChange={setHeight}>
                                    <Label>Height (mm)</Label>
                                    <NumberField.Input/>
                                </NumberField>
                            </div>

                            <NumberField minValue={0} maxValue={4} value={gap} onChange={setGap}>
                                <Label>Gap (mm)</Label>
                                <NumberField.Input/>
                            </NumberField>

                            <div className={"flex flex-row gap-4"}>
                                <NumberField minValue={0} maxValue={4} value={linerLeft} onChange={setLinerLeft}>
                                    <Label>Liner left (mm)</Label>
                                    <NumberField.Input/>
                                </NumberField>
                                <NumberField minValue={0} maxValue={4} value={linerRight} onChange={setLinerRight}>
                                    <Label>Liner right (mm)</Label>
                                    <NumberField.Input/>
                                </NumberField>
                            </div>
                            {errorMessage ?
                                <p className={"text-danger font-semibold text-sm-plus inline-flex items-center gap-2"}><Icon icon={"zondicons:exclamation-outline"}/> {errorMessage}</p>
                                : null
                            }
                        </Modal.Body>
                        <Modal.Footer>
                            <Button slot="close" size={"lg"} variant={"outline"}>
                                Cancel
                            </Button>
                            <Button
                                className={"primary"}
                                size={"lg"}
                                isDisabled={!isValidFormEntry}
                                onPress={() =>
                                {
                                    save();
                                    onClose();
                                }}
                            >
                                Save Stock
                            </Button>
                        </Modal.Footer>
                    </Modal.Dialog>
                </Modal.Container>
            </Modal.Backdrop>

        </Modal>
    );
}

type LabelSilhouetteProps = {
    width: number,
    height: number,
    className?: string,
}

export function LabelSilhouette({width, height, className}: LabelSilhouetteProps)
{

    return (
        <div className={cn("flex items-center justify-center grow-0 shrink-0 basis-auto bg-surface-alt border-[1.5px] border-dashed border-border-strong rounded-[10px] p-4", className)}>
            <div
                className={"bg-accent-soft border-[1.5px] border-accent"}
                style={{
                    aspectRatio: `${width} / ${height}`,
                    ...(width >= height
                        ? {width: "100%", maxHeight: "100%"}
                        : {height: "100%", maxWidth: "100%"})
                }}
            />
        </div>
    );
}


type LabelStockModalContextType = {
    isOpen: boolean,
    open: (stock?: LabelStock) => void,
    close: () => void,
}

const LabelStockModalContext = createContext<LabelStockModalContextType | undefined>(undefined);

export function LabelStockModalProvider({children}: { children: ReactNode })
{
    const [isOpen, setIsOpen] = useState(false);
    const [stock, setStock] = useState<LabelStock | undefined>(undefined);
    const open = (stock?: LabelStock | undefined) =>
    {
        setStock(stock);
        setIsOpen(true);
    };
    const close = () => setIsOpen(false);

    return (
        <LabelStockModalContext.Provider value={{isOpen, open, close}}>
            <LabelStockModal key={isOpen ? `open-${stock?.id ?? "new"}` : "closed"} isOpen={isOpen} onClose={close} labelStock={stock}/>
            {children}
        </LabelStockModalContext.Provider>
    );
}

export function useLabelStockModal(): LabelStockModalContextType
{
    const context = useContext(LabelStockModalContext);
    if (!context)
    {
        throw new Error("useLabelStockModal must be used within a LabelStockModalProvider");
    }
    return context;
}