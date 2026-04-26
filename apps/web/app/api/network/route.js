import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

export async function GET(req) {
  const xff = req.headers.get("x-forwarded-for") || "";
  const ip = xff.split(",")[0].trim() || "unknown";
  return NextResponse.json({ ip });
}
