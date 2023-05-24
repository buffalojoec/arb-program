import { useWallet } from '@solana/wallet-adapter-react'
import ArbCard from '@/components/ArbCard'
import { FC } from 'react'
import Image from 'next/image'

export const HomeView: FC = () => {
    const wallet = useWallet()

    return (
        <div className="md:hero mx-auto p-4">
            <div className="md:hero-content flex flex-col">
                <div className="mt-6">
                    <h1 className="mb-4 text-center text-4xl font-bold font-serif text-cyan-600">
                        Ready to pillage some ports?
                    </h1>
                </div>
                {wallet ? (
                    <div className="flex items-stretch justify-center space-x-10">
                        <div className="flex items-center justify-center flex-1">
                            <Image
                                className="rounded-2xl"
                                alt="arb"
                                src="/arb2.png"
                                width="550"
                                height="550"
                            />
                        </div>
                        <div className="flex items-center justify-center flex-1">
                            <ArbCard />
                        </div>
                    </div>
                ) : (
                    <div>
                        <h3 className="mb-4 mt-6 text-center text-2xl font-bold font-serif text-stone-500">
                            Connect a wallet to start pillaging
                        </h3>
                    </div>
                )}
            </div>
        </div>
    )
}
