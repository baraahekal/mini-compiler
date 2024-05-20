import { Box, Text } from "@chakra-ui/react";

const Output = (props) => {
    const { vars, lists } = props.parserData;
    return (
        <Box w="30%">
            <Box
                height="75vh"
                p={2}
                m={10}
                border="1px solid"
                borderRadius={5}
                borderColor= "#A0C9CB"
                backgroundColor="#282828"
            >
                {Object.entries(vars).map(([variable, value]) => (
                    <Text key={variable}>{`${variable}: ${value}`}</Text>
                ))}
                {Object.entries(lists).map(([list, values]) => (
                    <Text key={list}>{`${list} [${values.join(', ')}]`}</Text>
                ))}
            </Box>
        </Box>
    );
};
export default Output;