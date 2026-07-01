import {Button, Table} from "@heroui/react";
import {Icon} from "@iconify-icon/react";

export function LabelStocksPage()
{
    return (
        <div className={"flex justify-center w-full"}>
            <div className={"flex flex-col gap-2 mx-8 mt-5 w-full max-w-4xl"}>
                <div className={"flex flex-row gap-8"}>
                    <div className={"flex flex-col grow"}>
                        <h1 className={"font-bold text-display tracking-[-0.02em]"}>Label stocks</h1>
                        <p className={"font-light text-fg-muted mt-0.75 text-base-plus"}>Named presets for each label roll. Operators pick a stock by name — these<br/>dimensions tell the helper how to size and place the print.</p>
                    </div>
                    <Button size={"lg"} className={"mt-auto"}> <Icon icon={"lucide:plus"}/> Add stock</Button>
                </div>

                <Table>
                    <Table.Content>
                        <Table.Header>
                            <Table.Column>Icon</Table.Column>
                            <Table.Column>Name</Table.Column>
                            <Table.Column>Height</Table.Column>
                            <Table.Column>Gap</Table.Column>
                            <Table.Column>Liner L</Table.Column>
                            <Table.Column>Liner R</Table.Column>
                        </Table.Header>
                        <Table.ScrollContainer>
                            <Table.Body>
                                <Table.Row>
                                    <Table.Cell></Table.Cell>
                                    <Table.Cell></Table.Cell>
                                    <Table.Cell></Table.Cell>
                                    <Table.Cell></Table.Cell>
                                    <Table.Cell></Table.Cell>
                                    <Table.Cell></Table.Cell>
                                </Table.Row>
                            </Table.Body>
                        </Table.ScrollContainer>
                    </Table.Content>
                </Table>
            </div>
        </div>
    );
}