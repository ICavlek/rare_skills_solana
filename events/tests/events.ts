import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Events } from "../target/types/events";

describe("events", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Events as Program<Events>;

  it("Is initialized!", async () => {
    // Event names have to be camel case -> MyEvent not working, but myEvent does
    const listenerMyEvent = program.addEventListener('myEvent', (event, slot) => {
      console.log(`slot ${slot} event value ${event.value}`);
    });

    const listenerMySecondEvent = program.addEventListener('mySecondEvent', (event, slot) => {
      console.log(`slot ${slot} event value ${event.value} event message ${event.message}`);
    });

    await program.methods.initialize().rpc();

    await new Promise((resolve) => setTimeout(resolve, 5000));

    program.removeEventListener(listenerMyEvent);
    program.removeEventListener(listenerMySecondEvent);
  });
});
