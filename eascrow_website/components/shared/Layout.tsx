import React, { ReactNode } from 'react';
import Header from './Header';

interface LayoutProps {
  children: ReactNode;
}

const Layout = ({ children }: LayoutProps) => {
  return (
    <>
      <Header />
      <main className="mx-4 sm:mx-8 space-y-14">{children}</main>
    </>
  );
};

export default Layout;
