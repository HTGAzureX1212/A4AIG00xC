def mul_table(row_count: int):
    header = "   "
    for i in range(1, row_count + 1):
        header += f"{i}".rjust(10)
    print(header)

    for i in range(1, row_count + 1):
        string = f"{i}".rjust(3)
        for j in range(1, row_count + 1):
            string += f"{i * j}".rjust(10)

        print(string)


if __name__ == "__main__":
    failed = True

    while failed:
        try:
            rows_str = input("Enter the desired number of rows for the multiplication table: ")
            rows = int(rows_str)
            failed = False

            mul_table(rows)
        except ValueError:
            print("Invalid number. Please try again.")
