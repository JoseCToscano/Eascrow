import React, { ReactNode } from 'react';
import {
  NavigationMenu,
  NavigationMenuLink,
} from '@/components/ui/navigation-menu';
import Image from 'next/image';
import Link from 'next/link';
import MenuBurger from './MenuBurger';
import { Button } from '../ui/button';

interface HeaderProps {
  children?: ReactNode;
}

const Header: React.FC<HeaderProps> = ({ children }) => {
  return (
    <header className="flex justify-between items-center mb-10">
      <div>
        <Link href="/">
          <Image
            src="/logo-white.png"
            alt="Eascrow website"
            width="250"
            height="30"
            priority
            className="sm:hidden"
          />
          <Image
            src="/logo-white.png"
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
        <Button className="hidden sm:block bg-neonMintGreen text-xl text-darkBlueGray py-3 h-fit rounded-lg hover:bg-transparent hover:text-neonMintGreen">
          Launch DApp
        </Button>
      </div>
    </header>
  );
};

export default Header;
