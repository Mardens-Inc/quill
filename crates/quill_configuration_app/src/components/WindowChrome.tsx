
import {Button, ButtonGroup} from "@heroui/react";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {Icon} from "@iconify-icon/react";

export default function Navigation()
{
    const appWindow = getCurrentWindow();
    return (

        <div className={"flex flex-row h-10 backdrop-blur-sm sticky top-0 w-full z-51 backdrop-saturate-150 select-none bg-accent/20 text-white"} data-tauri-drag-region="">
            <div className={"flex flex-row"}>
                <p className={"mx-2 mt-1 text-lg font-bold select-none uppercase"} data-tauri-drag-region="">Quill</p>
            </div>
            <div className={"flex flex-row ml-auto"}>
                <ButtonGroup className={"h-8"} variant={"tertiary"}>
                    <Button variant={"ghost"} className={"min-w-0 h-8 rounded-none text-[1rem] hover:bg-gray-300"} onPress={() => appWindow.minimize()}><Icon icon="material-symbols:minimize-rounded"/></Button>
                    <Button variant={"ghost"} className={"min-w-0 h-8 rounded-none text-[.7rem] hover:bg-gray-300"} onPress={() => appWindow.toggleMaximize()}><Icon icon="material-symbols:square-outline-rounded"/></Button>
                    <Button variant={"ghost"} className={"min-w-0 h-8 rounded-none text-[1rem] hover:bg-danger hover:text-danger-foreground"} onPress={() => appWindow.close()}><Icon icon="material-symbols:close-rounded"/></Button>
                </ButtonGroup>
            </div>
        </div>
    );
}

