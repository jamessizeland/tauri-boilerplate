import React, { useState, useEffect } from 'react';
import {
  Button,
  Modal,
  ModalBody,
  ModalHeader,
  ModalFooter,
} from 'components/Elements';
import { useToggle } from 'hooks';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { ExampleData } from 'types';

const TestPage = (): JSX.Element => {
  // useEffect(() => {
  //   const unlisten = listen<number>('update_score', (event) => {
  //     console.log(event);
  //     setScore(event.payload);
  //   });
  // }, []);

  const [example, setExample] = useState<ExampleData>();
  const [message, setMessage] = useState('');
  const { isOpen, toggle } = useToggle();
  return (
    <div className="animate-backInRight animate-fast">
      <h1 className="text-3xl font-bold underline text-center">Test Page</h1>
      <div className="py-5 mt-5">
        <Modal toggle={toggle} isOpen={isOpen} animate position="extraLarge">
          <ModalHeader>Popup</ModalHeader>
          <ModalBody>
            Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do
            eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
            ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut
            aliquip ex ea commodo consequat.
          </ModalBody>
          <ModalFooter>
            <Button onClick={() => toggle(true)}>Modal</Button>
          </ModalFooter>
        </Modal>
        <div className="flex justify-center m-5">
          <Button onClick={() => toggle(false)}>Modal</Button>
        </div>
        <div className="flex justify-center text-center border border-blue-700 rounded-md">
          <div>
            <p>Tailwind Responsive Style Example</p>
            <p className="inline sm:hidden">Extra Small</p>
            <p className="hidden sm:inline md:hidden">Small</p>
            <p className="hidden md:inline lg:hidden">Medium</p>
            <p className="hidden lg:inline xl:hidden">Large</p>
            <p className="hidden xl:inline 2xl:hidden">Extra Large</p>
            <p className="hidden 2xl:inline">2X Large</p>
          </div>
        </div>
        <div className="flex justify-center m-5">
          <input
            type="text"
            placeholder="Type here"
            className="input input-bordered w-full max-w-xs"
            onBlur={(event) => setMessage(event.target.value)}
          />
          <Button
            onClick={() =>
              invoke<ExampleData>('mod_state', { message }).then((example) => {
                console.log(example);
                setExample(example);
              })
            }
          >
            Trigger Event
          </Button>
        </div>
        <div className="flex justify-center">
          {example && <p>{JSON.stringify(example)}</p>}
        </div>
      </div>
    </div>
  );
};

export default TestPage;
