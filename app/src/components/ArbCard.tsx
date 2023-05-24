import React, { useState } from 'react'

const MyComponent = () => {
    const [swap1, setSwap1] = useState('')
    const [swap2, setSwap2] = useState('')
    const [concurrency, setConcurrency] = useState(2)
    const [temperature, setTemperature] = useState(1)

    const handleSwap1Change = (e: {
        target: { value: React.SetStateAction<string> }
    }) => {
        setSwap1(e.target.value)
    }

    const handleSwap2Change = (e: {
        target: { value: React.SetStateAction<string> }
    }) => {
        setSwap2(e.target.value)
    }

    const handleConcurrencyChange = (e: { target: { value: string } }) => {
        const value = parseInt(e.target.value)
        setConcurrency(value)
    }

    const handleTemperatureChange = (e: { target: { value: string } }) => {
        const value = parseInt(e.target.value)
        setTemperature(value)
    }

    const launchArbitrage = () => {}

    return (
        <div className="w-full h-full rounded-2xl bg-stone-800 p-4">
            <div className="mt-4">
                <label className="block text-sm font-medium text-slate-400">
                    Swap #1
                </label>
                <input
                    type="text"
                    className="mt-1 p-2 block w-full border border-gray-700 rounded focus:ring-blue-500 focus:border-blue-500 bg-slate-800 text-white"
                    value={swap1}
                    onChange={handleSwap1Change}
                />
            </div>

            <div className="mt-4">
                <label className="block text-sm font-medium text-slate-400">
                    Swap #2
                </label>
                <input
                    type="text"
                    className="mt-1 p-2 block w-full border border-gray-700 rounded focus:ring-blue-500 focus:border-blue-500 bg-slate-800 text-white"
                    value={swap2}
                    onChange={handleSwap2Change}
                />
            </div>

            <div className="mt-6 ml-20 flex flex-row">
                <label className="my-auto text-sm font-medium text-lime-300">
                    Concurrency
                </label>
                <input
                    type="number"
                    className="w-1/3 ml-10 my-auto p-2 block border border-gray-700 rounded focus:ring-blue-500 focus:border-blue-500 bg-slate-800 text-white"
                    min={2}
                    max={20}
                    value={concurrency}
                    onChange={handleConcurrencyChange}
                />
            </div>

            <div className="mt-6 ml-20 flex flex-row">
                <label className="my-auto text-sm font-medium text-lime-300">
                    Temperature
                </label>
                <input
                    type="number"
                    className="w-1/3 ml-10 my-auto p-2 block border border-gray-700 rounded focus:ring-blue-500 focus:border-blue-500 bg-slate-800 text-white"
                    min={1}
                    max={99}
                    value={temperature}
                    onChange={handleTemperatureChange}
                />
            </div>

            <div className="mt-4">
                <button
                    className="w-full bg-cyan-700 hover:bg-cyan-900 h-12 mt-2 rounded-lg"
                    onClick={launchArbitrage}
                >
                    Launch Arbitrage
                </button>
            </div>
        </div>
    )
}

export default MyComponent
