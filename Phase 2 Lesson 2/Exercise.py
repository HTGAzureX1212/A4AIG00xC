def mul_table():
    print('   1  2  3  4')
    
    for i in range(4):
        string = f"{i + 1}"
        
        for j in range(4):
            string += f"  {(i + 1) * (j + 1)}"
        
        print(string)

if __name__ == "__main__":
    mul_table()
