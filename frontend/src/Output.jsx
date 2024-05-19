import { Box, Text } from "@chakra-ui/react";

const Output = ({ declaredVariables }) => {
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
                {Object.entries(declaredVariables).map(([variable, value]) => (
                    <Text key={variable}>{`${variable}: ${value}`}</Text>
                ))}
            </Box>
        </Box>
    );
};
export default Output;