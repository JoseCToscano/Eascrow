import React, { ReactNode } from 'react';
import {
  NavigationMenu,
  NavigationMenuLink,
} from '@/components/ui/navigation-menu';
import Image from 'next/image';
import Link from 'next/link';
import MenuBurger from './MenuBurger';
import ShimmerButton from '@/components/magicui/shimmer-button';

interface HeaderProps {
  children?: ReactNode;
}

const Header: React.FC<HeaderProps> = ({ children }) => {
  return (
    <header className="sticky top-0 left-4 w-full p-4 mb-4 flex justify-between items-center backdrop-blur-lg brightness-100 z-10">
      <div>
        <Link href="/">
          <Image
            src="/logo-white.avif"
            alt="Eascrow website"
            width="230"
            height="30"
            priority
            className="sm:hidden"
          />
          <Image
            src="/logo-white.avif"
            alt="Eascrow website"
            width="150"
            height="30"
            priority
            className="hidden sm:block"
          />
        </Link>
      </div>
      <div>
        <NavigationMenu className="hidden sm:block text-slate-400 text-lg space-x-6">
          <NavigationMenuLink href="/about" className="hover:opacity-60">
            About
          </NavigationMenuLink>
          <NavigationMenuLink href="/" className="hover:opacity-60">
            Feature
          </NavigationMenuLink>
          <NavigationMenuLink href="/" className="hover:opacity-60">
            Pricing
          </NavigationMenuLink>
        </NavigationMenu>
      </div>
      <div>
        <MenuBurger />
        <Link href="/dapp" className="hidden sm:block">
          <ShimmerButton
            className="shadow-2xl hover:opacity-90 "
            background="#5cffb8"
            shimmerSize="1px"
            shimmerColor="#001122"
            borderRadius="13px"
          >
            <span className=" whitespace-pre-wrap text-center text-sm font-medium leading-none tracking-tight dark:from-white dark:to-slate-900/10 lg:text-lg text-bgDark">
              Launch DApp
            </span>
          </ShimmerButton>
        </Link>
      </div>
    </header>
  );
};

export default Header;
