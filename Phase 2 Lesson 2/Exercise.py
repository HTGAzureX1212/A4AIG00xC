def mul_table():
    print('   1  2  3  4')

    for i in range(1, 5):
        string = f"{i}"
        for j in range(1, 5):
            string += f"{i * j}".rjust(3)

        print(string)


if __name__ == "__main__":
    mul_table()
