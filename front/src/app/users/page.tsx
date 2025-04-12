import { env } from "@/env";
import { Data } from "@/error";
import { User } from "@/types/user";
import { Card, Heading, Spinner, Text } from "@radix-ui/themes";
import Link from "next/link";
import { Suspense } from "react";

async function getUsers(): Promise<Data<User[]>> {
  try {
    const response = await fetch(`${env.API_URL}/users`, {
      next: {
        revalidate: 60,
      },
    });

    if (!response.ok) {
      throw new Error();
    }

    const users: User[] | undefined = await response.json();

    if (!users) {
      throw new Error();
    }

    return {
      status: "success",
      data: users,
    };
  } catch {
    return {
      status: "error",
      message: "ユーザーの取得に失敗しました。",
    };
  }
}

export default async function Page() {
  return (
    <>
      <Heading>ユーザー一覧</Heading>
      <Suspense fallback={<Spinner size="3" className="mx-auto mt-16" />}>
        <UserList />
      </Suspense>
    </>
  );
}

async function UserList() {
  const data = await getUsers();

  if (data.status === "error") {
    return <Text color="red">{data.message}</Text>;
  }

  const { data: users } = data;
  if (users.length === 0) {
    return <Text>ユーザーが見つかりませんでした。</Text>;
  }

  return (
    <div className="grid gap-4">
      {users.map((user) => (
        <Card asChild key={user.id}>
          <Link
            href={`/users/${user.id}`}
            aria-label={`${user.name}のプロフィールへ`}
          >
            <Text as="p" size="2" weight="bold">
              {user.name}
            </Text>
            <Text as="p" color="gray" size="2">
              {user.email}
            </Text>
          </Link>
        </Card>
      ))}
    </div>
  );
}
