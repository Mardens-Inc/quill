import {Button, Popover, Table, Tooltip} from "@heroui/react";
import {Icon} from "@iconify-icon/react";
import {useQuillSettings} from "../providers/QuillSettingsProvider.tsx";
import {LabelSilhouette, useLabelStockModal} from "../components/label_stocks/LabelStockModal.tsx";
import {useState} from "react";
import {testPrint} from "../printer.ts";

function DeletePopover({onConfirm}: { onConfirm: () => void })
{
    const [isOpen, setIsOpen] = useState(false);
    return (
        <Popover isOpen={isOpen} onOpenChange={setIsOpen}>
            <Popover.Trigger>
                <Button isIconOnly variant={"danger-soft"} className={"mx-1 rounded-lg"} size={"sm"}>
                    <Icon icon={"lucide:trash"}/>
                </Button>
            </Popover.Trigger>
            <Popover.Content>
                <Popover.Dialog className={"px-4 py-2 flex flex-col gap-2"}>
                    <p className={"text-fg-muted font-semibold text-base"}>Are you sure you want to delete this stock?</p>
                    <div className={"flex flex-row gap-2 mx-auto"}>
                        <Button onPress={() =>
                        {
                            onConfirm();
                            setIsOpen(false);
                        }} variant={"danger"}>Delete!</Button>
                        <Button variant={"outline"} onPress={() => setIsOpen(false)}>Cancel</Button>
                    </div>
                </Popover.Dialog>
            </Popover.Content>
        </Popover>
    );
}

export function LabelStocksPage()
{
    const {settings, deleteLabel} = useQuillSettings();
    const {open} = useLabelStockModal();
    return (
        <div className={"flex justify-center w-full"}>
            <div className={"flex flex-col gap-2 mx-8 mt-5 w-full max-w-4xl"}>
                <div className={"flex flex-row gap-8"}>
                    <div className={"flex flex-col grow"}>
                        <h1 className={"font-bold text-display tracking-[-0.02em]"}>Label stocks</h1>
                        <p className={"font-light text-fg-muted mt-0.75 text-base-plus"}>Named presets for each label roll. Operators pick a stock by name — these<br/>dimensions tell the helper how to size and place the print.</p>
                    </div>
                    <Button size={"lg"} className={"mt-auto primary"} onPress={() => open()}> <Icon icon={"lucide:plus"}/> Add stock</Button>
                </div>

                <Table>
                    <Table.ScrollContainer>
                        <Table.Content aria-label={"Label stock table"}>
                            <Table.Header>
                                <Table.Column></Table.Column>
                                <Table.Column isRowHeader>NAME</Table.Column>
                                <Table.Column>WIDTH</Table.Column>
                                <Table.Column>HEIGHT</Table.Column>
                                <Table.Column>GAP</Table.Column>
                                <Table.Column>LINER L</Table.Column>
                                <Table.Column>LINER R</Table.Column>
                                <Table.Column minWidth={160}></Table.Column>
                            </Table.Header>
                            <Table.Body>
                                {settings.labels.map(label =>
                                    <Table.Row>
                                        <Table.Cell><LabelSilhouette width={label.width} height={label.height} className={"w-9.5 h-7.5 p-1 rounded-sm"}/></Table.Cell>
                                        <Table.Cell className={"font-semibold text-base"}>{label.name}</Table.Cell>
                                        <Table.Cell className={"font-mono text-fg-muted text-md"}>{label.width}</Table.Cell>
                                        <Table.Cell className={"font-mono text-fg-muted text-md"}>{label.height}</Table.Cell>
                                        <Table.Cell className={"font-mono text-fg-muted text-md"}>{label.gap}</Table.Cell>
                                        <Table.Cell className={"font-mono text-fg-muted text-md"}>{label.linerL}</Table.Cell>
                                        <Table.Cell className={"font-mono text-fg-muted text-md"}>{label.linerR}</Table.Cell>
                                        <Table.Cell>
                                            <Tooltip closeDelay={0} delay={1000}>
                                                <Tooltip.Trigger>
                                                    <Button
                                                        isIconOnly
                                                        variant={"outline"}
                                                        className={"mx-1 rounded-lg"}
                                                        size={"sm"}
                                                        onPress={() => open(label)}
                                                    >
                                                        <Icon icon={"lucide:edit"}/>
                                                    </Button>
                                                </Tooltip.Trigger>
                                                <Tooltip.Content>Edit the label stock</Tooltip.Content>
                                            </Tooltip>
                                            <Tooltip closeDelay={0} delay={1000}>
                                                <Tooltip.Trigger>
                                                    <Button
                                                        isIconOnly
                                                        variant={"outline"}
                                                        className={"mx-1 rounded-lg"}
                                                        size={"sm"}
                                                        onPress={() => testPrint(label)}
                                                    >
                                                        <Icon icon={"lucide:printer"}/>
                                                    </Button>
                                                </Tooltip.Trigger>
                                                <Tooltip.Content>Start a test print</Tooltip.Content>
                                            </Tooltip>
                                            <Tooltip closeDelay={0} delay={1000}>
                                                <Tooltip.Trigger>
                                                    <DeletePopover onConfirm={() => deleteLabel(label.id!)}/>
                                                </Tooltip.Trigger>
                                                <Tooltip.Content>Delete the label stock</Tooltip.Content>
                                            </Tooltip>
                                        </Table.Cell>
                                    </Table.Row>
                                )}
                            </Table.Body>
                        </Table.Content>
                    </Table.ScrollContainer>
                    <Table.Footer>
                        <p className={"text-sm-plus text-fg-subtle"}>All dimensions in millimetres. Gap is the vertical space between labels; liner offsets account for backing-paper margins.</p>
                    </Table.Footer>
                </Table>
            </div>
        </div>
    );
}