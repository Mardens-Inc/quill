import React from "react";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import ReactDOM from "react-dom/client";

import "./css/index.css";
import PrinterSetupPage from "./pages/PrinterSetupPage.tsx";
import WindowChrome from "./components/WindowChrome.tsx";
import {Toast} from "@heroui/react";
import {attachConsoleToTracing} from "./util/logger.ts";
import {QuillSettingsProvider} from "./providers/QuillSettingsProvider.tsx";
import {SidePanel} from "./components/SidePanel.tsx";
import {LabelStocksPage} from "./pages/LabelStocksPage.tsx";
import {PrintSettingsPage} from "./pages/PrintSettingsPage.tsx";
import {ServerSecurityPage} from "./pages/ServerSecurityPage.tsx";
import {LogsPage} from "./pages/LogsPage.tsx";
import {AboutPage} from "./pages/AboutPage.tsx";

// Route all console output and uncaught errors through the Rust tracing
// pipeline so frontend logs land in the same rolling log files as native logs.
attachConsoleToTracing();

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <BrowserRouter>
            <QuillSettingsProvider>
                <MainContentRenderer/>
            </QuillSettingsProvider>
        </BrowserRouter>
    </React.StrictMode>
);

export function MainContentRenderer()
{
    window.addEventListener("message", (e) => e.preventDefault());
    return (
        <>
            <Toast.Provider placement={"bottom end"}/>
            <main className={"flex flex-col p-0 m-0"}>
                <WindowChrome/>

                <div className={"flex flex-row w-full max-h-[calc(100vh-48px)] h-screen overflow-y-hidden p-0 m-0 gap-2"}>
                    <SidePanel/>
                    <Routes>
                        <Route>
                            <Route path="/" element={<PrinterSetupPage/>}/>
                            <Route path="/stocks" element={<LabelStocksPage/>}/>
                            <Route path="/print-settings" element={<PrintSettingsPage/>}/>
                            <Route path="/security" element={<ServerSecurityPage/>}/>
                            <Route path="/logs" element={<LogsPage/>}/>
                            <Route path="/about" element={<AboutPage/>}/>
                        </Route>
                    </Routes>
                </div>

            </main>
        </>
    );
}
