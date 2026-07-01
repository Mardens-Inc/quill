import {ErrorBoundary} from "../ErrorBoundry.tsx";
import {Button, Description, Input, ListBox} from "@heroui/react";
import {useQuillSettings} from "../providers/QuillSettingsProvider.tsx";
import {Icon} from "@iconify-icon/react";
import {useState} from "react";

export function ServerSecurityPage()
{
    const {settings, setListenPort, setAllowedOrigins} = useQuillSettings();
    const [newOrigin, setNewOrigin] = useState("");

    const addOrigin = () =>
    {
        const trimmed = newOrigin.trim();
        if (!trimmed || settings.allowedOrigins.includes(trimmed)) return;
        setAllowedOrigins([...settings.allowedOrigins, trimmed]);
        setNewOrigin("");
    };

    const removeOrigin = (origin: string) =>
    {
        setAllowedOrigins(settings.allowedOrigins.filter(o => o !== origin));
    };

    return (
        <div className={"flex justify-center w-full"}>
            <div className={"flex flex-col gap-2 mx-8 mt-5 w-full max-w-4xl"}>
                <div className={"flex flex-col grow"}>
                    <h1 className={"font-bold text-display tracking-[-0.02em]"}>Server &amp; security</h1>
                    <p className={"font-light text-fg-muted mt-0.75 text-base-plus"}>The helper exposes a tiny local API the company web app calls to print. It listens on loopback only.</p>
                </div>

                <ErrorBoundary>
                    <div className={"flex flex-col bg-surface p-6 rounded-2xl shadow-sm border"}>
                        <label htmlFor={"listen_port"} className="text-base-plus font-bold">Listening port</label>
                        <Description>The helper binds to this port on the loopback address only. The value must be between 1023 and 65535</Description>
                        <div className={"w-full bg-surface rounded-lg border flex flex-row items-center overflow-hidden font-mono my-2"}>
                            <div className={"bg-surface-alt text-fg-muted text-base h-10.5 items-center flex px-3.25 border-r border-input"}>127.0.0.1</div>
                            <input
                                id={"listen_port"}
                                value={settings.helperServicePort}
                                onChange={e =>
                                {
                                    let value = Number(e.target.value.replace(/[^0-9]/g, ""));
                                    if (value > 65535) value = 65535;
                                    if (value < 1023) value = 1023;
                                    setListenPort(value);
                                }}
                                className={"text-base font-mono text-fg bg-input px-3.25 focus:outline-none! focus-visible:outline-none! w-full"}
                            />
                        </div>
                        <div className={"flex flex-row px-3.5 py-2.75 text-sm-plus leading-[1.45] rounded-lg bg-info text-info-fg items-center gap-2.25"}>
                            <Icon icon={"zondicons:exclamation-outline"} className={"text-xl"}/>
                            <p>Loopback only — <b>not exposed to the network</b>. Nothing outside this laptop can reach the helper.</p>
                        </div>
                    </div>
                    <div className={"flex flex-col bg-surface p-6 rounded-2xl shadow-sm border"}>
                        <label htmlFor={"allowed_origins"} className="text-base-plus font-bold">Allowed origins</label>
                        <Description>Only these web app origins may call the helper. Requests from anywhere else are rejected.</Description>
                        <ListBox>
                            {settings.allowedOrigins.length === 0 ? <ListBox.Item isDisabled>No Items</ListBox.Item> :
                                <>
                                    {settings.allowedOrigins.map(item =>
                                        <ListBox.Item key={item} className={"bg-surface-alt h-10.5 rounded-lg mb-2 border border-border"}>
                                            <p className={"w-full font-mono text-md"}>{item}</p>
                                            <Button variant={"ghost"} size={"sm"} onPress={() => removeOrigin(item)}>
                                                <Icon icon={"lucide:x"}/>
                                            </Button>
                                        </ListBox.Item>
                                    )}
                                </>
                            }
                        </ListBox>
                        <div className={"flex flex-row gap-4"}>
                            <Input
                                id={"allowed_origins"}
                                className={"w-full"}
                                placeholder={"https://*.example.com"}
                                value={newOrigin}
                                onChange={e => setNewOrigin(e.target.value)}
                                onKeyDown={e => e.key === "Enter" && addOrigin()}
                            />
                            <Button variant={"outline"} onPress={addOrigin}><Icon icon={"lucide:plus"}/>Add</Button>
                        </div>
                    </div>
                    <div className={"flex flex-row bg-surface p-6 rounded-2xl shadow-sm border justify-between"}>
                        <div className={"flex flex-col gap-2"}>

                            <label className="text-base-plus font-bold">Restart helper service</label>
                            <Description>Applies port and security changes. Printing is briefly unavailable during restart.</Description>
                        </div>
                        <Button variant={"outline"} size={"lg"}><Icon icon={"tabler:reload"}/> Restart helper</Button>
                    </div>
                </ErrorBoundary>
            </div>
        </div>
    );
}