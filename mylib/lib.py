def get_median(df):
    value_list = df.values.tolist()
    
    value_list.sort()

    # Step 2: Calculate the median
    n = len(value_list)
    
    if n % 2 == 0:  # If the list has an even number of elements
        middle1 = n // 2
        middle2 = middle1 - 1
        median = (value_list[middle1] + value_list[middle2]) / 2
    else:  # If the list has an odd number of elements
        middle = n // 2
        median = value_list[middle]
    
    return median

