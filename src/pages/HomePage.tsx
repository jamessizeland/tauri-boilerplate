import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
// import { listen } from '@tauri-apps/api/event';
import { Button } from 'components/Elements';
import { checkEnv } from 'utils';
import { useToggle } from 'hooks';
import clsx from 'clsx';

const HomePage = (): JSX.Element => {
  const { isOpen, toggle } = useToggle();
  // listen<Position>('update_position', (event) => {
  //   console.log('update position');
  //   setPosition(event.payload);
  // });

  useEffect(() => {
    // do this when the page mounts
    toggle(false);
  }, []);

  return (
    <div className="animate-backInRight animate-fast">
      <h1 className="text-3xl font-bold underline text-center">Home</h1>
      <div className="flex justify-around items-center flex-wrap mt-5 py-5 border border-black">
        <p>Main Div</p>
      </div>
      <div className="pt-5 w-full flex">Second Div</div>
    </div>
  );
};
export default HomePage;
