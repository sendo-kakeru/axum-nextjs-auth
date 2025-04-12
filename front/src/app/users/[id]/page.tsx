import { env } from "@/env";
import { Data } from "@/error";
import { User as UserType } from "@/types/user";
import { PageProps } from "@/types/utils";
import { Heading, Text, Link, Spinner } from "@radix-ui/themes";
import { notFound } from "next/navigation";
import NextLink from "next/link";
import { Suspense } from "react";

async function getUser(id: string): Promise<Data<UserType>> {
  try {
    const response = await fetch(`${env.API_URL}/users/${id}`);
    const user: UserType | undefined = await response.json();

    if (response.status === 404) {
      notFound();
    }
    if (!response.ok || !user) {
      return {
        status: "error",
        message: "ユーザーの取得に失敗しました。",
      };
    }
    return {
      status: "success",
      data: user,
    };
  } catch {
    return {
      status: "error",
      message: "ユーザーの取得に失敗しました。",
    };
  }
}

export default async function Page(props: PageProps<"id">) {
  return (
    <>
      <Heading>ユーザー詳細</Heading>
      <Suspense fallback={<Spinner size="3" className="mx-auto mt-16" />}>
        <User {...props} />
      </Suspense>
      <Link asChild>
        <NextLink href="/users">ユーザー一覧へ戻る</NextLink>
      </Link>
    </>
  );
}

async function User({ params }: PageProps<"id">) {
  const { id } = await params;
  const data = await getUser(id);
  if (data.status === "error") {
    return <Text color="red">{data.message}</Text>;
  }
  const user = data.data;

  return (
    <>
      <Heading>{user.name}</Heading>
      <Text>ユーザーID: {user.id}</Text>
      <Text>メールアドレス: {user.email}</Text>
    </>
  );
}
