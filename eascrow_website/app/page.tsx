import Link from 'next/link';
import { Button } from '@/components/ui/button';
import Image from 'next/image';
import { Separator } from '@/components/ui/separator';
import { Card, CardContent, CardTitle } from '@/components/ui/card';
import { Box, ContactRound, Truck } from 'lucide-react';

export default function Home() {
  return (
    <>
      <div className="text-center">
        <h1 className="text-4xl font-bold">
          When the trusted third is the{' '}
          <span className="text-neonMintGreen block my-4">blockchain</span>
        </h1>
        <p className="text-slate-400 mb-10">
          Eascrow leverages Soroban to provide secured escrow services for web3,
          guaranteeing fast and fraud-resistant payments.
        </p>
        <Link href="/dapp" className="">
          <Button className="px-6 py-7 text-md rounded-lg bg-custom-gradient border border-slate-700 ">
            Get started now
          </Button>
        </Link>
      </div>
      <section className="flex flex-col items-center space-y-4">
        <div className="w-full max-w-4xl rounded-[25px] border-gray-700 overflow-hidden">
          <Image
            src="/dashboard.png"
            alt="Dashboard overview"
            width={800}
            height={450}
            layout="responsive"
          />
        </div>
      </section>
      <div className="flex flex-col items-center text-slate-400">
        <div className="bg-bgDark items-center flex flex-col">
          <section className="w-full max-w-lg h-64 flex flex-col justify-center space-y-1">
            <p className="text-xl text-center text-slate-400">Trusted by</p>
            <div className="flex justify-evenly">
              <div>
                <Image
                  src="/stellar-logo.avif"
                  alt="Dashboard overview"
                  width={130}
                  height={10}
                />
              </div>
              <div className="flex flex-col justify-center">
                <Image
                  src="/soroban-logo.avif"
                  alt="Dashboard overview"
                  width={130}
                  height={50}
                />
              </div>
            </div>
          </section>
          <Separator className="w-3/4 bg-gray-700 mb-10" />
          <section className="flex flex-col items-center">
            <div className="mb-4">
              <Image
                src="/about-image.avif"
                alt="Dashboard overview"
                width={350}
                height={10}
              />
            </div>
            <div className="text-center flex flex-col items-center px-3">
              <h2 className="text-customWhite text-4xl font-bold mb-3">
                About
              </h2>
              <p>
                Eascrow is an innovative web3 escrow service that harnesses the
                power of Soroban smart contracts to streamline and secure online
                transactions. It acts as a trusted intermediary, ensuring that
                funds are safely held and released according to the terms agreed
                upon by the involved parties. Our service is essential for
                various financial dealings, ranging from material goods trade to
                freelance services.
              </p>
              <Separator className="w-3/4 bg-gray-700 my-20" />
              <h2 className="text-customWhite text-4xl font-bold mb-3">
                Features
              </h2>
              <p>
                The Problem: Traditional online transactions often face
                challenges like fraud, disputes over contract terms,
                inefficiencies in fund transfer, especially in international
                settings. These issues erode trust and complicate the
                transaction process.
              </p>
              <Separator className="w-3/4 bg-gray-700 my-20" />
            </div>
            <div className="w-full px-2 space-y-6 lg:space-y-0 flex flex-col lg:flex-row items-center lg:justify-evenly">
              <Card className="max-w-xs bg-transparent border-gray-700 rounded-[30px] w-full h-[400px] shadow-inner flex flex-col items-center text-center">
                <div className="w-16 h-16 shadow-lg shadow-black rounded-full flex justify-center items-center bg-[#212b38] my-10">
                  <Box color="#5cffb8" />
                </div>
                <CardTitle className="text-customWhite mb-4">
                  For individuals and Businesses
                </CardTitle>
                <CardContent className="text-slate-400">
                  <p>
                    Eascrow reduces fraud risk by securely holding funds until
                    confirmation of goods' delivery and satisfaction is
                    received. This streamlines the transaction process,
                    providing assurance to both buyers and sellers.
                  </p>
                </CardContent>
              </Card>
              <Card className="max-w-xs bg-transparent border-gray-700 rounded-[30px] w-full h-[400px] shadow-inner flex flex-col items-center text-center">
                <div className="w-16 h-16 shadow-lg shadow-black rounded-full flex justify-center items-center bg-[#212b38] my-10">
                  <ContactRound color="#5cffb8" />
                </div>
                <CardTitle className="text-customWhite mb-4">
                  For Freelancers and Employers
                </CardTitle>
                <CardContent className="text-slate-400">
                  <p>
                    It ensures secure and streamlined payment for services,
                    holding funds until the completion of agreed work. This
                    simplifies the payment process and offers peace of mind to
                    both parties.
                  </p>
                </CardContent>
              </Card>
              <Card className="max-w-xs bg-transparent border-gray-700 rounded-[30px] w-full h-[400px] shadow-inner flex flex-col items-center text-center">
                <div className="w-16 h-16 shadow-lg shadow-black rounded-full flex justify-center items-center bg-[#212b38] my-10">
                  <Truck color="#5cffb8" />
                </div>
                <CardTitle className="text-customWhite mb-4">
                  For International Transactions
                </CardTitle>
                <CardContent className="text-slate-400">
                  <p>
                    Eascrow simplifies and secures cross-border transactions
                    using Soroban smart contracts. It streamlines compliance
                    with international regulations, handling complexities like
                    currency exchange and legal requirements.
                  </p>
                </CardContent>
              </Card>
            </div>
          </section>
        </div>
      </div>
    </>
  );
}
