import {ErrorBoundary} from "../ErrorBoundry.tsx";
import {useQuillSettings} from "../providers/QuillSettingsProvider.tsx";
import {Accordion, ListBox, Select, Separator} from "@heroui/react";
import {Icon} from "@iconify-icon/react";

export function PrintSettingsPage()
{
    const {
        settings,
        setDensity,
        setPrintIps,
        setDefaultOrientation,
        setScale,
        setMonochromeThreshold
    } = useQuillSettings();
    return (
        <div className={"flex justify-center w-full"}>
            <div className={"flex flex-col gap-2 mx-8 mt-5 w-full max-w-4xl"}>
                <div className={"flex flex-col grow"}>
                    <h1 className={"font-bold text-display tracking-[-0.02em]"}>Print settings</h1>
                    <p className={"font-light text-fg-muted mt-0.75 text-base-plus"}>Defaults applied to every print job. Operators can't change these — tune them once for clean, durable labels.</p>
                </div>

                <ErrorBoundary>
                    <div className={"flex flex-col bg-surface p-6 rounded-2xl shadow-sm border"}>
                        <div className="flex items-center justify-between mb-1.5">
                            <label className="text-base-plus font-bold">Darkness / density</label>
                            <span className="font-mono text-lg font-semibold text-accent min-w-7 text-right">{settings.density}</span>
                        </div>
                        <input
                            type="range"
                            min={0}
                            max={15}
                            step={1}
                            value={settings.density}
                            onChange={e => setDensity(Number(e.target.value))}
                            className="w-full my-2.5 mb-1"
                        />
                        <div className="flex justify-between text-xs text-fg-subtle font-mono mt-2">
                            <span onClick={() => setDensity(0)} className={"cursor-pointer"}>0 · light</span>
                            <span onClick={() => setDensity(8)} className={"cursor-pointer"}>8 · recommended</span>
                            <span onClick={() => setDensity(15)} className={"cursor-pointer"}>15 · dark</span>
                        </div>
                        <p className="text-sm-plus text-fg-muted mt-2.5 leading-snug">Higher values burn darker. Too high smears on synthetic stock; too low fades on thermal paper. Most stock prints clean at 8–10.</p>
                    </div>
                    <div className={"flex flex-row gap-6 "}>
                        <div className={"flex flex-col bg-surface p-6 rounded-2xl shadow-sm border grow shrink basis-0"}>
                            <label className="text-base-plus font-bold">Print speed</label>
                            <Select
                                aria-label="Print speed"
                                value={settings.printIps.toString()}
                                onChange={value => setPrintIps(Number(value))}
                            >
                                <Select.Trigger>
                                    <Select.Value/>
                                    <Select.Indicator/>
                                </Select.Trigger>
                                <Select.Popover>
                                    <ListBox aria-label="Print speed options">
                                        <ListBox.Item id={"2"} key={"2"} textValue={"2"}>2 ips - slowest, sharpest</ListBox.Item>
                                        <ListBox.Item id={"4"} key={"4"} textValue={"4"}>4 ips - recommended</ListBox.Item>
                                        <ListBox.Item id={"6"} key={"6"} textValue={"6"}>6 ips - fast</ListBox.Item>
                                        <ListBox.Item id={"8"} key={"8"} textValue={"8"}>8 ips - fastest</ListBox.Item>
                                    </ListBox>
                                </Select.Popover>
                            </Select>
                            <p className="text-sm-plus text-fg-muted mt-2.5 leading-snug">Slower speeds give crisper barcodes. Drop to 2–4 ips if scanners struggle to read printed codes.</p>
                        </div>
                        <div className={"flex flex-col bg-surface p-6 rounded-2xl shadow-sm border grow shrink basis-0"}>
                            <label className="text-base-plus font-bold">Default orientation</label>
                            {(() =>
                            {
                                // Values 0-3 are the quarter-turn presets (Normal/90/180/270);
                                // any other value is a custom clockwise angle in degrees.
                                const isCustom = ![0, 1, 2, 3].includes(settings.defaultOrientation);
                                const selectedKey = isCustom ? "custom" : settings.defaultOrientation.toString();
                                // Custom degrees are stored in a u8 and must avoid the 0-3 preset codes.
                                const clampDegrees = (value: number): number =>
                                    Number.isFinite(value) ? Math.min(255, Math.max(4, Math.round(value))) : 45;
                                return (
                                    <>
                                        <Select
                                            aria-label="Default orientation"
                                            className="w-full max-w-md mt-2"
                                            value={selectedKey}
                                            onChange={key => setDefaultOrientation(
                                                key === "custom"
                                                    ? (isCustom ? settings.defaultOrientation : 45)
                                                    : Number(key)
                                            )}
                                        >
                                            <Select.Trigger>
                                                <Select.Value/>
                                                <Select.Indicator/>
                                            </Select.Trigger>
                                            <Select.Popover>
                                                <ListBox aria-label="Default orientation options">
                                                    <ListBox.Item id={"0"} key={"0"} textValue={"Normal (0°)"}>Normal (0°)</ListBox.Item>
                                                    <ListBox.Item id={"1"} key={"1"} textValue={"Rotate 90°"}>Rotate 90° clockwise</ListBox.Item>
                                                    <ListBox.Item id={"2"} key={"2"} textValue={"Rotate 180°"}>Rotate 180°</ListBox.Item>
                                                    <ListBox.Item id={"3"} key={"3"} textValue={"Rotate 270°"}>Rotate 270° clockwise</ListBox.Item>
                                                    <ListBox.Item id={"custom"} key={"custom"} textValue={"Custom angle"}>Custom angle…</ListBox.Item>
                                                </ListBox>
                                            </Select.Popover>
                                        </Select>
                                        {isCustom && (
                                            <div className="mt-3">
                                                <div className="flex items-center justify-between mb-1.5">
                                                    <span className="text-sm-plus text-fg-muted">Custom angle</span>
                                                    <span className="font-mono text-lg font-semibold text-accent min-w-7 text-right">{settings.defaultOrientation}°</span>
                                                </div>
                                                <input
                                                    type="range"
                                                    aria-label="Custom rotation in degrees"
                                                    min={4}
                                                    max={255}
                                                    step={1}
                                                    value={settings.defaultOrientation}
                                                    onChange={e => setDefaultOrientation(clampDegrees(Number(e.target.value)))}
                                                    className="w-full my-2.5 mb-1"
                                                />
                                                <div className="flex justify-between text-xs text-fg-subtle font-mono mt-2">
                                                    <span onClick={() => setDefaultOrientation(4)} className={"cursor-pointer"}>4°</span>
                                                    <span onClick={() => setDefaultOrientation(45)} className={"cursor-pointer"}>45°</span>
                                                    <span onClick={() => setDefaultOrientation(255)} className={"cursor-pointer"}>255°</span>
                                                </div>
                                            </div>
                                        )}
                                    </>
                                );
                            })()}
                            <p className="text-sm-plus text-fg-muted mt-2.5 leading-snug">Rotation applied before printing. Presets cover the common quarter-turns; use a custom angle (4–255°) for anything else.</p>
                        </div>
                    </div>

                    <div className={"flex flex-col bg-surface p-6 rounded-2xl shadow-sm border"}>
                        <div className="flex items-center justify-between mb-1.5">
                            <label className="text-base-plus font-bold">Default scale</label>
                            <span className="font-mono text-lg font-semibold text-accent min-w-7 text-right">{settings.scale}%</span>
                        </div>
                        <input
                            type="range"
                            min={50}
                            max={150}
                            step={1}
                            value={settings.scale}
                            onChange={e => setScale(Number(e.target.value))}
                            className="w-full my-2.5 mb-1"
                        />
                        <div className="flex justify-between text-xs text-fg-subtle font-mono mt-2">
                            <span onClick={() => setScale(50)} className={"cursor-pointer"}>50%</span>
                            <span onClick={() => setScale(100)} className={"cursor-pointer"}>100%</span>
                            <span onClick={() => setScale(150)} className={"cursor-pointer"}>150%</span>
                        </div>
                        <p className="text-sm-plus text-fg-muted mt-2.5 leading-snug">Scales label content. Keep at 100% unless artwork is consistently over- or undersized.</p>
                    </div>

                    <Accordion className="w-full bg-surface rounded-2xl shadow border">
                        <Accordion.Item key={0}>
                            <Accordion.Heading>
                                <Accordion.Trigger className={"justify-start"}>
                                    <Accordion.Indicator className={"ml-0 text-lg text-fg"}>
                                        <Icon icon={"lucide:chevron-down"}/>
                                    </Accordion.Indicator>
                                    <p className={"text-base-plus font-bold pl-2"}>Advanced</p>
                                </Accordion.Trigger>
                            </Accordion.Heading>
                            <Separator/>
                            <Accordion.Panel>
                                <Accordion.Body>
                                    <div className="flex items-center justify-between mb-1.5 mt-6">
                                        <label className="text-base font-bold text-fg">Monochrome threshold</label>
                                        <span className="font-mono text-lg font-semibold text-accent min-w-7 text-right">{settings.monochromeThreshold}</span>
                                    </div>
                                    <input
                                        type="range"
                                        min={0}
                                        max={255}
                                        step={1}
                                        value={settings.monochromeThreshold}
                                        onChange={e => setMonochromeThreshold(Number(e.target.value))}
                                        className="w-full my-2.5 mb-1"
                                    />
                                    <div className="flex justify-between text-xs text-fg-subtle font-mono mt-2">
                                        <span onClick={() => setMonochromeThreshold(0)} className={"cursor-pointer w-24 text-start"}>0</span>
                                        <span onClick={() => setMonochromeThreshold(128)} className={"cursor-pointer w-24 text-center"}>128</span>
                                        <span onClick={() => setMonochromeThreshold(255)} className={"cursor-pointer w-24 text-end"}>255</span>
                                    </div>
                                    <p className="text-sm-plus text-fg-muted mt-2.5 leading-snug">Pixels darker than this become black; lighter become white when converting color artwork for thermal printing. Raise it to keep faint detail, lower it to drop background noise.</p>
                                </Accordion.Body>
                            </Accordion.Panel>
                        </Accordion.Item>
                    </Accordion>
                </ErrorBoundary>
            </div>
        </div>
    );
}