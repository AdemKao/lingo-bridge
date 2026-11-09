# Set up native LINE desktop audio routing

**This is an experimental real-audio implementation, not an official LINE integration.**
The Japanese-speaking participant uses an ordinary LINE client and does not install Lingo Bridge.

## You need TWO separate virtual audio cables

You need two *independent* virtual endpoints; one cable shared for both directions causes feedback.

| Channel | LINE Desktop audio setting | Lingo Bridge audio setting |
| --- | --- | --- |
| Cable A (outgoing translation) | **Microphone:** Cable A's recording/input endpoint | **③ Japanese output:** Cable A's playback/output endpoint |
| Cable B (incoming speech) | **Speaker:** Cable B's playback/output endpoint | **② LINE incoming capture:** Cable B's recording/input endpoint |
| Physical microphone | Must **not** be selected in LINE | **① My microphone** |
| Physical headphones | Do not directly play the untranslated LINE signal | **④ My headphones** |

### Windows

Example virtual audio cable provider: https://vb-audio.com/Cable/index.htm . A cable's **CABLE Input** is usually a playback device and **CABLE Output** is a recording device; this naming is counterintuitive. Use two separate cable instances with appropriate licensing. Confirm both appear in Windows sound settings and LINE Desktop.

### macOS

Example virtual device: https://github.com/ExistentialAudio/BlackHole . Configure **two independent devices** (or create appropriately separate named virtual endpoints) and verify the routing in Audio MIDI Setup. You might need an additional routing tool or custom BlackHole instances. A single shared two-channel device is **not** sufficient to isolate both directions.

## How to test without consuming API credit

1. Plug in wired/wireless headphones to avoid acoustic feedback.
2. Set LINE microphone to Cable A's *recording* side and LINE speakers to Cable B's *playback* side.
3. Start a LINE call with a consenting Japanese-speaking participant.
4. Select all four Lingo Bridge devices.
5. Click **Play 0.5-second test tone**. Ask the remote participant whether they hear a short beep. The app sends no real speech and uses no API key for this test.
6. If the test fails, verify cable directions. Never select your physical microphone in LINE.
7. Confirm Cable B is receiving incoming LINE audio in system sound meters.
8. Enter your own API key, acknowledge informed consent, click **Start translation**, and test both directions.

## Known caveats

- Available device IDs are based on OS enumeration order and may change when devices are connected or disconnected. Refresh device listing when that happens.
- Both incoming and outgoing LINE original audio must be isolated from translated playback. This version is **manual device routing** rather than automatic per-app capture.
- LINE's echo suppression may interact with the virtual devices. Using headphones is recommended.
- This experimental code relies on the default device sample format (f32/i16/u16) and prototype linear resampling. Other formats may need an updated adapter.
- No automatic virtual driver installer is bundled. Driver distribution, code signing and licensing are future work.
- Do not assume Windows/macOS/LINE compatibility without direct hardware testing.
- If translation fails, stop the call and investigate rather than changing to the real microphone unknowingly.

## Input signal diagnostics (no cloud processing)

The desktop UI includes **Test my microphone** and **Test LINE incoming audio**. Each button explicitly opens the selected input for about one second and reports RMS/peak signal levels **without saving or sending samples to AI**. Ask the remote participant's permission before testing call audio. The LINE incoming Cable B signal should rise while the remote participant speaks. If it stays near silence, revisit the LINE speaker and Cable B endpoint mapping before using paid AI translation. Real call compatibility still needs both-direction hardware tests.
