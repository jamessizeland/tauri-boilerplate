import React from 'react';

const AboutPage = (): JSX.Element => (
  <div className="animate-backInRight animate-fast">
    <h1 className="text-3xl font-bold underline text-center">About Page</h1>
    <div className="justify-center mx-auto flex mt-5 py-5">
      <div className="text-left">
        <ul>
          <li>Uses fork of Kimia-UI library</li>
          <li>Built using Tauri and Rust for the backend</li>
          <li>Built using Vitejs TypeScript React for the frontend</li>
        </ul>
      </div>
    </div>
  </div>
);

export default AboutPage;
