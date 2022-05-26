import { Formik, Field } from "formik";
import { useEffect , useState} from 'react';

import {
    Box,
    Button,
    Flex,
    FormControl,
    FormLabel,
    FormErrorMessage,
    Input,
    VStack,
    Textarea,
    Stat,
    StatLabel,
    StatNumber,
} from "@chakra-ui/react";
import { providers, utils } from "ethers"
import * as Yup from "yup";

const provider = new providers.JsonRpcProvider("http://localhost:8545")

const SignupSchema = Yup.object().shape({
    name: Yup.string()
        .min(2, 'Too Short!')
        .max(50, 'Too Long!')
        .required('Required'),
    age: Yup.number()
        .min(1, 'Too Short!')
        .max(150, 'Too Long!')
        .required('Required'),
    address: Yup.string().min(20, 'Too Short for an address').max(200, 'Too long').required('Required'),
});

export default function Chakra() {
    const [ev, setEv] = useState('');

    useEffect(()=>{
        let filter = {
            address: "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512",
            topics: [utils.id("NewGreeting(bytes32)")]
        }
        provider.on(filter, (result) => {
            console.log(result);
            setEv(utils.parseBytes32String(result.data) + ` on Block #${result.blockNumber}`);
        });
    }, []);

    return (
        <Flex bg="gray.100" align="center" justify="center" h="100vh">
            <Box bg="white" p={6} rounded="md" w={64}>
                <Formik
                    initialValues={{
                        name: "",
                        age: "",
                        address: ""
                    }}
                    onSubmit={(values) => {
                        console.log(JSON.stringify(values, null, 2));
                    }}
                    validationSchema={SignupSchema}
                >
                    {({ handleSubmit, errors, touched }) => (
                        <form onSubmit={handleSubmit}>
                            <VStack spacing={4} align="flex-start">
                                <FormControl isInvalid={!!errors.name && touched.name}>
                                    <FormLabel htmlFor="name">Full Name</FormLabel>
                                    <Field
                                        as={Input}
                                        id="name"
                                        name="name"
                                        type="text"
                                        variant="filled"
                                    />
                                    <FormErrorMessage>{errors.name}</FormErrorMessage>

                                </FormControl>
                                <FormControl isInvalid={!!errors.age && touched.age}>
                                    <FormLabel htmlFor="age">Age</FormLabel>
                                    <Field
                                        as={Input}
                                        id="age"
                                        name="age"
                                        type="number"
                                        variant="filled"
                                    />
                                    <FormErrorMessage>{errors.age}</FormErrorMessage>
                                </FormControl>
                                <FormControl isInvalid={!!errors.address && touched.address}>
                                    <FormLabel htmlFor="address">Address</FormLabel>
                                    <Field
                                        as={Textarea}
                                        id="address"
                                        name="address"
                                        type="text"
                                        variant="filled"
                                    />
                                    <FormErrorMessage>{errors.address}</FormErrorMessage>
                                </FormControl>
                                <Button type="submit" colorScheme="purple" width="full">
                                    Submit
                                </Button>
                                <hr />
                                <Stat>
                                    <StatLabel>Latest Greet : </StatLabel>
                                    <StatNumber>{ev}</StatNumber>
                                </Stat>

                            </VStack>
                        </form>
                    )}
                </Formik>
            </Box>
        </Flex>
    );
}