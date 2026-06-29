import iconLight from "../../../../res/icons/quill-dark.svg";
import {openUrl} from "@tauri-apps/plugin-opener";

export function AboutPage()
{
    return (
        <div className={"flex justify-center w-full"}>
            <div className={"flex flex-col gap-2 mx-8 mt-5 w-full max-w-4xl"}>
                <h1 className={"font-bold text-display"}>About</h1>
                <p className={"font-light text-fg-muted mt-0.75"}>Version and support information for the Quill helper on this laptop.</p>
                <div className={"flex flex-row gap-4 bg-surface p-6 rounded-2xl shadow-sm border"}>
                    <img src={iconLight} alt={"logo"} className={"h-14 w-14"}/>
                    <div className={"flex flex-col gap-2"}>
                        <p className={"text-2xl font-bold"}>Quill Configurator</p>
                        <p className={"text-sm-plus text-fg-muted"}>Admin setup tool for the Quill label-printing helper.</p>
                    </div>
                </div>
                <div className={"flex flex-col bg-surface rounded-2xl shadow-sm border"}>
                    <div className={"flex flex-row justify-between border-b px-5.5 py-3.75 items-center"}>
                        <p className={"text-md-plus text-fg-muted"}>App version</p>
                        <p className={"text-md-plus text-fg font-mono font-semibold"}>2.4.1</p>
                    </div>
                    <div className={"flex flex-row justify-between border-b px-5.5 py-3.75 items-center"}>
                        <p className={"text-md-plus text-fg-muted"}>Helper service</p>
                        <p className={"text-md-plus text-fg font-mono font-semibold"}>2.4.1 - build 1180</p>
                    </div>
                    <div className={"flex flex-row justify-between border-b px-5.5 py-3.75 items-center"}>
                        <p className={"text-md-plus text-fg-muted"}>Config schema</p>
                        <p className={"text-md-plus text-fg font-mono font-semibold"}>v7</p>
                    </div>
                    <div className={"flex flex-row justify-between border-b px-5.5 py-3.75 items-center"}>
                        <p className={"text-md-plus text-fg-muted"}>Helper status</p>
                        <p className={"text-md-plus text-fg font-mono font-semibold"}>
                            <div className={"flex flex-row items-center justify-center rounded-full px-2 py-1 bg-success-soft text-success-soft-foreground font-bold text-sm-plus"}>
                                <span className={"w-1.5 h-1.5 bg-success rounded-full mx-1"}/>Running
                            </div>
                        </p>
                    </div>
                </div>
                <div className={"flex flex-col bg-surface rounded-2xl shadow-sm border p-6"}>
                    <p className={"font-bold text-md-plus mb-1.5"}>Support</p>
                    <p className={"text-md text-fg-muted"}>IT support - <span onClick={() => openUrl("mailto:helpdesk@mardens.com")} className={"text-accent hover:cursor-pointer hover:underline"}>helpdesk@mardens.com</span></p>
                    <p className={"text-md text-fg-muted"}>Internal Docs - <span onClick={() => openUrl("https://pricetagger.mardens.com/docs/quill")} className={"text-accent hover:cursor-pointer hover:underline"}>pricetagger.mardens.com/docs/quill</span></p>
                    <p className={"text-fg-muted mt-0.75 text-md"}>When reporting an issue, export logs from Diagnostics and attach the file above.</p>
                </div>
            </div>
        </div>
    );
}