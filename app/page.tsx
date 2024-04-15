"use client";

import React, {useState} from 'react';
import {invoke} from '@tauri-apps/api/tauri';

export default function Home() {
    const [markdown, setMarkdown] = useState("");
    const [bbcode, setBBCode] = useState("");

    function handleMarkdownChange(e: { target: { value: React.SetStateAction<string>; }; }) {
        setMarkdown(e.target.value);
    }

    async function handleConvert() {
        const bbcodeResult = await invoke('convert_to_bbcode', {markdownText: markdown});
        setBBCode(bbcodeResult as string);
    }

    return (
        <main className="flex min-h-screen flex-col items-center justify-between p-24">
            <div className="z-10 w-full max-w-5xl items-center justify-between font-mono text-sm lg:flex">
                <div>
                    <p>Markdown Input:</p>
                    <textarea value={markdown} onChange={handleMarkdownChange}/>
                </div>


                <div>
                    <p>BBCode Output:</p>
                    <textarea value={bbcode} readOnly/>
                </div>
                <button onClick={handleConvert}>Convert</button>
            </div>
        </main>
    );
}