"use client";

import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export default function Home() {
    const [markdown, setMarkdown] = useState("");
    const [bbcode, setBBCode] = useState("");

    function handleMarkdownChange(e: { target: { value: React.SetStateAction<string>; }; }) {
        setMarkdown(e.target.value);
    }

    async function handleConvert() {
        const bbcodeResult = await invoke('convert_to_bbcode', { markdownText: markdown });
        setBBCode(bbcodeResult as string);
    }

    return (
        <main className="flex min-h-screen flex-col items-center justify-center bg-gray-100 p-10 space-y-6">
            <div className="space-x-4 md:space-x-12 w-11/12 md:w-3/4 flex flex-col md:flex-row">
                <div className="w-full md:w-1/2 bg-white p-5 rounded-lg">
                    <h2 className="font-mono text-lg mb-4">Markdown Input:</h2>
                    <textarea
                        className="w-full h-72 p-3 rounded-lg bg-gray-200 resize-none"
                        value={markdown}
                        onChange={handleMarkdownChange}
                    />
                </div>

                <div className="w-full mt-6 md:mt-0 md:w-1/2 bg-white p-5 rounded-lg">
                    <h2 className="font-mono text-lg mb-4">BBCode Output:</h2>
                    <textarea
                        className="w-full h-72 p-3 rounded-lg bg-gray-200 resize-none"
                        value={bbcode}
                        readOnly

                    />
                </div>
            </div>
            <button
                className="py-2 px-4 text-white bg-blue-500 rounded-lg hover:bg-blue-600"
                onClick={handleConvert}
            >
                Convert
            </button>
        </main>
    );
}