import {Icon} from "@iconify-icon/react";
import {useNavigate} from "react-router-dom";
import {cn} from "@heroui/styles";
import {useAbout} from "../providers/AboutProvider.tsx";
import {useQuillSettings} from "../providers/QuillSettingsProvider.tsx";
import {Separator} from "@heroui/react";

export function SidePanel()
{
    const {settings} = useQuillSettings();
    const {configSchemaVersion} = useAbout();
    return (
        <div className={"flex flex-col w-56.75 bg-sidebar border-r p-4 gap-2 shrink-0 grow-0"}>
            <SidePanelItem label={"Printer Setup"} icon={"lucide:printer"} href={"/"}/>
            <SidePanelItem label={"Label Stocks"} icon={"lucide:tag"} href={"/stocks"}/>
            <SidePanelItem label={"Print Settings"} icon={"lucide:sliders"} href={"/print-settings"}/>
            <SidePanelItem label={"Server & Security"} icon={"lucide:shield"} href={"/security"}/>
            <SidePanelItem label={"Logs & Diagnostics"} icon={"lucide:list"} href={"/logs"}/>
            <SidePanelItem label={"About"} icon={"lucide:info"} href={"/about"}/>
            <div className={"mt-auto font-mono font-light text-xs-plus text-fg-subtle"}>
                <p>Config schema <b>v{configSchemaVersion}</b></p>
                <Separator/>
                <p className={"mt-2"}>Loopback only: <br/>
                    <span className={"font-bold"}>127.0.0.1:{settings.helperServicePort}</span>
                </p>
            </div>
        </div>
    );
}

type SidePanelItemProps = {
    label: string,
    icon: string,
    href: string,
}

function SidePanelItem({label, href, icon}: SidePanelItemProps)
{
    const navigate = useNavigate();
    return (
        <div
            className={cn(
                "flex flex-row gap-3.25 rounded-[11px] text-base px-3 h-11 items-center cursor-pointer",
                "data-[active=true]:bg-accent-sidebar data-[active=true]:font-semibold",
                "hover:data-[active=true]:bg-accent-sidebar/50",
                "hover:bg-surface-3/50 transition duration-200"
            )}
            onClick={() => navigate(href)}
            data-active={window.location.pathname == href}
        >
            <Icon icon={icon}/>
            <span>{label}</span>
        </div>
    );

}