import React from "react";
import { Button, H1 } from "@blueprintjs/core";
import init, { tani } from "@crate/tani_checker/pkg";

init();

export const App: React.FC = () => {
  return (
    <>
      <H1>Hello</H1>
      <Button onClick={() => tani("[[class]]\n  id = \"31HG122\"\n  name = \"English Reading Skills I\"\n  credits = 1.0")}>tani</Button>
    </>
  );
};
