import { Heading, Link } from "@radix-ui/themes";
import NextLink from "next/link";

export default async function Home() {
  return (
    <>
      <Heading>Home</Heading>
      <Link asChild>
        <NextLink href="/users">
          ユーザー一覧
        </NextLink>
      </Link>
    </>
  );
}
