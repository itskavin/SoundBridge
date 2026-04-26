import "./globals.css";

export const metadata = {
  title: "SoundBridge Web",
  description: "Peer-to-peer audio relay with same-network discovery",
};

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
