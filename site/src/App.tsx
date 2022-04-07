import React from "react";
import { Button, H1 } from "@blueprintjs/core";
import init, { tani } from "@crate/tani_checker/pkg";

init();

export const App: React.FC = () => {
  return (
    <>
      <H1>Hello</H1>
      <Button onClick={() => tani("hoge")}>tani</Button>
    </>
  );
};
