import { env } from "@/env";
import { Data } from "@/error";
import { User } from "@/types/user";
import { Heading, Spinner, Text } from "@radix-ui/themes";
import { Suspense } from "react";

async function getUsers(): Promise<Data<User[]>> {
  try {
    const response = await fetch(`${env.API_URL}/users`);
    const users: User[] | undefined = await response.json();

    if (!response.ok || !users) {
      return {
        status: "error",
        message: "ユーザーの取得に失敗しました。",
      };
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

export default async function Home() {
  return (
    <div className="grid gap-y-12 max-2 px-4 max-w-6xl mx-auto py-8 lg:py-16 h-full">
      <Heading>ユーザー一覧</Heading>
      <Suspense fallback={<Spinner size="3" className="mx-auto mt-16" />}>
        <UserList />
      </Suspense>
    </div>
  );
}

async function UserList() {
  const result = await getUsers();

  if (result.status === "error") {
    return <Text color="red">{result.message}</Text>;
  }

  if (result.data.length === 0) {
    return <Text>ユーザーが見つかりませんでした。</Text>;
  }

  return (
    <div className="grid gap-4">
      {result.data.map((user) => (
        <div
          key={user.id}
          className="border border-gray-300 p-4 rounded-lg shadow-sm"
        >
          <Text>{user.name}</Text>
          <Text>{user.email}</Text>
        </div>
      ))}
    </div>
  );
}
