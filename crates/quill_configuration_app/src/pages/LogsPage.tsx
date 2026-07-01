import {ErrorBoundary} from "../ErrorBoundry.tsx";
import {Button, ListBox, Select} from "@heroui/react";
import {Icon} from "@iconify-icon/react";
import {useCallback, useEffect, useMemo, useRef, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {openPath} from "@tauri-apps/plugin-opener";

type LogLevel = "trace" | "debug" | "info" | "warn" | "error";

/** Mirrors the `LogItem` struct returned by the `get_logs` Tauri command. */
type LogItem = {
    timestamp: string,
    level: string,
    fields: { message: string },
    target: string,
    threadName?: string,
};

type LogEntry = {
    time: string,
    level: LogLevel,
    module: string,
    message: string,
};

/** How often the log list re-fetches from the helper, in milliseconds. */
const REFRESH_INTERVAL_MS = 1000;

/** Format an ISO timestamp as `HH:MM:SS.mmm`, falling back to the raw string. */
function formatTime(timestamp: string): string
{
    const date = new Date(timestamp);
    if (isNaN(date.getTime())) return timestamp;
    const pad = (n: number, len = 2) => n.toString().padStart(len, "0");
    return `${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}.${pad(date.getMilliseconds(), 3)}`;
}

/** Coerce a backend level string (e.g. "DEBUG") to a known `LogLevel`. */
function normalizeLevel(level: string): LogLevel
{
    const lower = level.toLowerCase();
    return (LEVEL_ORDER as string[]).includes(lower) ? lower as LogLevel : "info";
}

/** Convert a raw backend `LogItem` into the shape the list renders. */
function toEntry(item: LogItem): LogEntry
{
    return {
        time: formatTime(item.timestamp),
        level: normalizeLevel(item.level),
        module: item.target,
        message: item.fields?.message ?? "",
    };
}

const LEVEL_STYLE: Record<LogLevel, { label: string, badgeBg: string, badgeFg: string, rail?: string, rowBg?: string }> = {
    trace: {label: "TRACE", badgeBg: "rgba(148,163,184,0.14)", badgeFg: "rgb(148,163,184)"},
    debug: {label: "DEBUG", badgeBg: "rgba(100,116,139,0.12)", badgeFg: "rgb(100,116,139)"},
    info: {label: "INFO", badgeBg: "rgba(37,99,235,0.1)", badgeFg: "rgb(37,99,235)"},
    warn: {label: "WARN", badgeBg: "rgba(217,119,6,0.13)", badgeFg: "rgb(180,83,9)", rail: "rgb(240,201,144)"},
    error: {label: "ERROR", badgeBg: "rgba(220,38,38,0.11)", badgeFg: "rgb(220,38,38)", rail: "rgb(240,179,168)", rowBg: "rgba(220,38,38,0.05)"},
};

const LEVEL_ORDER: LogLevel[] = ["trace", "debug", "info", "warn", "error"];

export function LogsPage()
{
    const [minLevel, setMinLevel] = useState<LogLevel>("trace");
    const [timeRange, setTimeRange] = useState("15m");
    const [search, setSearch] = useState("");
    const [activeLevels, setActiveLevels] = useState<Set<LogLevel>>(new Set(LEVEL_ORDER));
    const [liveTail, setLiveTail] = useState(false);
    const [remoteSubmission, setRemoteSubmission] = useState(false);
    const [logs, setLogs] = useState<LogEntry[]>([]);
    const [logsDir, setLogsDir] = useState("");
    const listRef = useRef<HTMLDivElement>(null);

    // Resolve the real logs directory once (it lives next to the executable).
    useEffect(() =>
    {
        invoke<string>("logs_directory").then(setLogsDir).catch(() => setLogsDir(""));
    }, []);

    const openLogsFolder = useCallback(() =>
    {
        if (logsDir) void openPath(logsDir).catch(() => {});
    }, [logsDir]);

    const refresh = useCallback(async () =>
    {
        try
        {
            const items: LogItem[] = await invoke("get_logs");
            setLogs(items.map(toEntry));
        } catch
        {
            // Keep the last successful snapshot on a failed poll.
        }
    }, []);

    // Fetch immediately, then poll on an interval so the list stays current.
    useEffect(() =>
    {
        void refresh();
        const id = window.setInterval(() => void refresh(), REFRESH_INTERVAL_MS);
        return () => window.clearInterval(id);
    }, [refresh]);

    const toggleLevel = (level: LogLevel) =>
    {
        setActiveLevels(prev =>
        {
            const next = new Set(prev);
            if (next.has(level)) next.delete(level);
            else next.add(level);
            return next;
        });
    };

    const filtered = useMemo(() =>
    {
        const minIndex = LEVEL_ORDER.indexOf(minLevel);
        const query = search.trim().toLowerCase();
        return logs.filter(entry =>
        {
            if (LEVEL_ORDER.indexOf(entry.level) < minIndex) return false;
            if (!activeLevels.has(entry.level)) return false;
            if (query && !entry.message.toLowerCase().includes(query) && !entry.module.toLowerCase().includes(query)) return false;
            return true;
        });
    }, [logs, minLevel, search, activeLevels]);

    // While live tail is on, pin the list to the bottom as new entries arrive.
    useEffect(() =>
    {
        if (!liveTail) return;
        const el = listRef.current;
        if (el) el.scrollTop = el.scrollHeight;
    }, [liveTail, filtered]);

    return (
        <div className={"flex justify-center w-full"}>
            <div className={"flex flex-col gap-2 mx-8 mt-5 w-full max-w-4xl"}>
                <div className={"flex flex-row items-end gap-4 mb-2.5"}>
                    <div className={"flex flex-col grow"}>
                        <h1 className={"font-bold text-display tracking-[-0.02em]"}>Logs &amp; diagnostics</h1>
                        <p className={"font-light text-fg-muted mt-0.75 text-base-plus"}>Live event stream from the helper. Filter, tail, and export when diagnosing a problem on this laptop.</p>
                    </div>
                    <div className={"flex flex-row items-center gap-2 shrink-0"}>
                        <label className={"text-sm-plus font-semibold text-fg-muted"}>Log level</label>
                        <Select
                            aria-label={"Minimum log level"}
                            value={minLevel}
                            onChange={value => setMinLevel(value as LogLevel)}
                        >
                            <Select.Trigger className={"h-9"}>
                                <Select.Value/>
                                <Select.Indicator/>
                            </Select.Trigger>
                            <Select.Popover>
                                <ListBox aria-label={"Minimum log level options"}>
                                    <ListBox.Item id={"trace"} key={"trace"} textValue={"Trace"}>Trace</ListBox.Item>
                                    <ListBox.Item id={"debug"} key={"debug"} textValue={"Debug"}>Debug</ListBox.Item>
                                    <ListBox.Item id={"info"} key={"info"} textValue={"Info"}>Info</ListBox.Item>
                                    <ListBox.Item id={"warn"} key={"warn"} textValue={"Warn"}>Warn</ListBox.Item>
                                    <ListBox.Item id={"error"} key={"error"} textValue={"Error"}>Error</ListBox.Item>
                                </ListBox>
                            </Select.Popover>
                        </Select>
                    </div>
                </div>

                <ErrorBoundary>
                    {/* Toolbar */}
                    <div className={"bg-surface border shadow-sm rounded-t-2xl px-4 py-3.5 flex flex-row items-center gap-3 flex-wrap"}>
                        <div className={"flex flex-row items-center gap-2 grow min-w-52 h-9.5 px-3 rounded-lg border border-input-border bg-input"}>
                            <Icon icon={"lucide:search"} className={"text-fg-subtle text-md"}/>
                            <input
                                placeholder={"Search messages and modules…"}
                                value={search}
                                onChange={e => setSearch(e.target.value)}
                                className={"grow bg-transparent border-0 outline-none text-fg text-md-plus focus:outline-none"}
                            />
                        </div>
                        <div className={"flex flex-row gap-1.5"}>
                            {LEVEL_ORDER.map(level =>
                            {
                                const style = LEVEL_STYLE[level];
                                const active = activeLevels.has(level);
                                return (
                                    <button
                                        key={level}
                                        onClick={() => toggleLevel(level)}
                                        className={"h-8 px-3 rounded-full text-sm font-semibold cursor-pointer border border-transparent transition-opacity"}
                                        style={{
                                            background: style.badgeBg,
                                            color: style.badgeFg,
                                            opacity: active ? 1 : 0.4,
                                        }}
                                    >
                                        {level.charAt(0).toUpperCase() + level.slice(1)}
                                    </button>
                                );
                            })}
                        </div>
                        <Select
                            aria-label={"Time range"}
                            value={timeRange}
                            onChange={value => setTimeRange(value as string)}
                        >
                            <Select.Trigger className={"h-8"}>
                                <Select.Value/>
                                <Select.Indicator/>
                            </Select.Trigger>
                            <Select.Popover>
                                <ListBox aria-label={"Time range options"}>
                                    <ListBox.Item id={"15m"} key={"15m"} textValue={"Last 15 min"}>Last 15 min</ListBox.Item>
                                    <ListBox.Item id={"1h"} key={"1h"} textValue={"Last hour"}>Last hour</ListBox.Item>
                                    <ListBox.Item id={"24h"} key={"24h"} textValue={"Last 24 h"}>Last 24 h</ListBox.Item>
                                    <ListBox.Item id={"all"} key={"all"} textValue={"All time"}>All time</ListBox.Item>
                                </ListBox>
                            </Select.Popover>
                        </Select>
                    </div>

                    {/* Status bar */}
                    <div className={"bg-surface-2 border-x border-border px-4 py-2.25 flex flex-row items-center gap-3"}>
                        <button
                            onClick={() => setLiveTail(v => !v)}
                            className={"inline-flex items-center gap-2 h-8 px-3.25 rounded-full text-sm-plus font-semibold cursor-pointer border transition-colors"}
                            style={{
                                borderColor: "var(--color-border-strong)",
                                background: liveTail ? "var(--color-success-soft)" : "transparent",
                                color: liveTail ? "var(--color-success-soft-foreground)" : "var(--color-fg-muted)",
                            }}
                        >
                            <span
                                className={"w-1.75 h-1.75 rounded-full"}
                                style={{
                                    background: liveTail ? "var(--color-success-soft-foreground)" : "var(--color-fg-subtle)",
                                    animation: liveTail ? "qpulse 1.4s ease-in-out infinite" : "none",
                                }}
                            />
                            Live tail
                        </button>
                        <span className={"text-sm text-fg-subtle font-mono"}>{filtered.length} of {logs.length} entries</span>
                        <div className={"grow"}/>
                        <Button variant={"outline"} size={"sm"} className={"h-8 gap-1.5 border-border text-fg-muted"}>
                            <Icon icon={"lucide:trash-2"} className={"text-sm"}/>
                            Clear
                        </Button>
                        <Button variant={"outline"} size={"sm"} className={"h-8 gap-1.5 border-border text-fg-muted"}>
                            <Icon icon={"lucide:download"} className={"text-sm"}/>
                            Export
                        </Button>
                    </div>

                    {/* Log list */}
                    <div ref={listRef} className={"bg-surface border shadow-sm rounded-b-2xl max-h-95 overflow-y-auto"}>
                        {filtered.length === 0 ?
                            <div className={"px-4 py-8 text-center text-sm-plus text-fg-subtle font-mono"}>No entries match the current filters.</div> :
                            filtered.map((entry, index) =>
                            {
                                const style = LEVEL_STYLE[entry.level];
                                return (
                                    <div
                                        key={index}
                                        className={"grid gap-3 items-start px-4 py-2.25 border-b border-border cursor-pointer border-l-[3px]"}
                                        style={{
                                            gridTemplateColumns: "104px 70px 160px 1fr",
                                            background: style.rowBg ?? "transparent",
                                            borderLeftColor: style.rail ?? "transparent",
                                        }}
                                    >
                                        <span className={"font-mono text-sm text-fg-subtle pt-px"}>{entry.time}</span>
                                        <span
                                            className={"inline-flex items-center justify-center h-4.75 rounded-md text-tiny font-bold tracking-[0.03em]"}
                                            style={{background: style.badgeBg, color: style.badgeFg}}
                                        >
                                            {style.label}
                                        </span>
                                        <span className={"font-mono text-sm text-fg-muted pt-px truncate min-w-0"} title={entry.module}>{entry.module}</span>
                                        <span className={"text-md text-fg font-mono leading-normal truncate min-w-0"} title={entry.message}>{entry.message}</span>
                                    </div>
                                );
                            })
                        }
                    </div>

                    {/* Remote log submission */}
                    <div className={"bg-surface border shadow-sm rounded-2xl p-6 mt-2.5"}>
                        <div className={"flex flex-row items-center gap-3.5"}>
                            <div className={"grow"}>
                                <div className={"text-base-plus font-bold"}>Remote log submission</div>
                                <div className={"text-sm-plus text-fg-muted mt-0.75 leading-snug"}>Forward logs to a central server so support can diagnose this laptop without remote access.</div>
                            </div>
                            <button
                                role={"switch"}
                                aria-checked={remoteSubmission}
                                onClick={() => setRemoteSubmission(v => !v)}
                                className={"w-11.5 h-6.75 rounded-full cursor-pointer p-0.75 flex items-center transition-colors"}
                                style={{
                                    background: remoteSubmission ? "var(--color-accent)" : "var(--color-border-strong)",
                                    justifyContent: remoteSubmission ? "flex-end" : "flex-start",
                                }}
                            >
                                <span className={"w-5.25 h-5.25 rounded-full bg-white"} style={{boxShadow: "rgba(0,0,0,0.4) 0 1px 3px"}}/>
                            </button>
                        </div>
                    </div>

                    {/* Log file location */}
                    <div className={"bg-surface border shadow-sm rounded-2xl px-6 py-5 mt-2.5 flex flex-row items-center gap-3.5 flex-wrap"}>
                        <div className={"grow min-w-55"}>
                            <div className={"text-md-plus font-bold mb-1"}>Log file location</div>
                            <div className={"font-mono text-sm-plus text-fg-muted break-words"}>{logsDir || "Resolving…"}</div>
                        </div>
                        <Button variant={"outline"} className={"rounded-full h-9.5 gap-1.75 shrink-0 bg-surface-2"} isDisabled={!logsDir} onPress={openLogsFolder}>
                            <Icon icon={"lucide:folder"} className={"text-base"}/>
                            Open folder
                        </Button>
                    </div>
                </ErrorBoundary>
            </div>
        </div>
    );
}
