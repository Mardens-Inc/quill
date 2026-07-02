import {LabelStock} from "./providers/QuillSettingsProvider.tsx";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "@heroui/react";

export async function testPrint(label: LabelStock)
{
    try
    {
        await invoke("create_test_print", {stockId: label.id});

        toast(`Test Print!`, {
            variant: "success",
            description: "Test print was a success!",
            actionProps: {
                children: "Dismiss",
                onPress: () => toast.clear(),
                variant: "tertiary"
            }
        });
    } catch (e: any)
    {
        toast(`Failed to create test print!`, {
            variant: "danger",
            description: e,
            actionProps: {
                children: "Dismiss",
                onPress: () => toast.clear(),
                variant: "tertiary"
            }
        });
    }
}