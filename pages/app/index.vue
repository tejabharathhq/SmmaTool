<script setup lang="ts">
import {
    Phone,
    Copy,
    CreditCard,
    File,
    ListFilter,
    MoreVertical,
    Truck,
} from 'lucide-vue-next'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import {
    Pagination,
    PaginationList,
    PaginationNext,
    PaginationPrev,
} from '@/components/ui/pagination'
import { Progress } from '@/components/ui/progress'
import { Separator } from '@/components/ui/separator'
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from '@/components/ui/table'
import {
    Tabs,
    TabsContent,
    TabsList,
    TabsTrigger,
} from '@/components/ui/tabs'
import { Checkbox } from '@/components/ui/checkbox'
import { LightningBoltIcon } from '@radix-icons/vue'
// Example array with type annotations for recent calls
const recentCalls: { contact: string; status: { label: string; variant: "default" | "destructive" | "secondary" | "outline" }; date: string }[] = [
    { contact: 'John Doe', status: { label: 'Booked', variant: 'default' }, date: '2024-04-27' },
    { contact: 'Bob Johnson', status: { label: 'Signed', variant: 'secondary' }, date: '2024-04-23' },
    { contact: 'Alice Smith', status: { label: 'Rejected', variant: 'outline' }, date: '2024-04-25' },
];


const upcomingEvents = [
    { name: 'Team Meeting', date: '2024-04-30', time: '10:00 AM', location: 'Discord' },
    { name: 'Product Launch', date: '2024-05-02', time: '2:00 PM', location: 'Zoom' },
    { name: 'Training Session', date: '2024-05-04', time: '9:00 AM', location: 'Google Meets' }
];

const upcomingTasks: { name: string; dueDate: string; status: { label: string; variant: "default" | "destructive" | "secondary" | "outline" } }[] = [
    { name: 'Prepare Presentation', dueDate: '2024-04-30', status: { label: 'Pending', variant: 'outline' } },
    { name: 'Review Documents', dueDate: '2024-05-01', status: { label: 'In Progress', variant: 'secondary' } },
    { name: 'Send Proposal', dueDate: '2024-05-03', status: { label: 'Completed', variant: 'default' } }
];


</script>

<template>
    <main class="grid items-start gap-4 p-4 sm:px-6 sm:py-4 md:gap-8 lg:grid-cols-3 xl:grid-cols-3">
        <div class="grid auto-rows-max items-start gap-4 md:gap-8 lg:col-span-2">
            <div class="grid gap-4 sm:grid-cols-2 md:grid-cols-4 lg:grid-cols-2 xl:grid-cols-4">
                <Card class="sm:col-span-2">
                    <CardHeader class="pb-3">
                        <CardTitle>Scraped Leads</CardTitle>
                        <CardDescription class="max-w-lg text-balance leading-relaxed">
                            Introducing Our Dynamic Lead Scraping Dashboard for Seamless
                            Lead Generation and Analysis.
                        </CardDescription>
                    </CardHeader>
                    <CardFooter>
                        <Button class="gap-x-2">
                            <LightningBoltIcon class="h-3.5 w-3.5"></LightningBoltIcon> Start Scraper
                        </Button> <!-- Change button text -->
                    </CardFooter>
                </Card>
                <Card>
                    <CardHeader class="pb-2">
                        <CardDescription>Leads Scraped</CardDescription> <!-- Change description -->
                        <CardTitle class="text-4xl">
                            500 <!-- Example number of leads scraped -->
                        </CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div class="text-xs text-muted-foreground">
                            +25% from last week <!-- Example increase percentage -->
                        </div>
                    </CardContent>
                    <CardFooter>
                        <Progress :model-value="25" aria-label="25% increase" /> <!-- Example progress -->
                    </CardFooter>
                </Card>
                <Card>
                    <CardHeader class="pb-2">
                        <CardDescription>Calls Made</CardDescription> <!-- Change description -->
                        <CardTitle class="text-3xl">
                            100 <!-- Example number of calls made -->
                        </CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div class="text-xs text-muted-foreground">
                            +10% from last month <!-- Example increase percentage -->
                        </div>
                    </CardContent>
                    <CardFooter>
                        <Progress :model-value="12" aria-label="12% increase" /> <!-- Example progress -->
                    </CardFooter>
                </Card>
            </div>
            <Tabs default-value="upcoming-events">
                <div class="flex items-center">
                    <TabsList>
                        <TabsTrigger value="upcoming-events">
                            Upcoming Events
                        </TabsTrigger>
                        <TabsTrigger value="recent-calls">
                            Recent Calls
                        </TabsTrigger>
                        <TabsTrigger value="upcoming-tasks">
                            Upcoming Tasks
                        </TabsTrigger>
                    </TabsList>
                    <div class="ml-auto flex items-center gap-2">
                        <DropdownMenu>
                            <DropdownMenuTrigger as-child>
                                <Button variant="outline" size="sm" class="h-7 gap-1 rounded-md px-3">
                                    <ListFilter class="h-3.5 w-3.5" />
                                    <span class="sr-only sm:not-sr-only">Filter</span>
                                </Button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent align="end">
                                <DropdownMenuLabel>Filter by</DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem>
                                    <div class="items-top flex space-x-2">
                                        <Checkbox id="fulfilled" />
                                        <label for="fulfilled"
                                            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                                            Fulfilled
                                        </label>
                                    </div>
                                </DropdownMenuItem>
                                <!-- Add more filter options here if needed -->
                            </DropdownMenuContent>
                        </DropdownMenu>
                        <Button variant="outline" size="sm" class="h-7 gap-1 rounded-md px-3">
                            <File class="h-3.5 w-3.5" />
                            <span class="sr-only sm:not-sr-only">Export</span>
                        </Button>
                    </div>
                </div>
                <TabsContent value="recent-calls">
                    <!-- Recent Calls Section -->
                    <Card>
                        <CardHeader class="px-7">
                            <CardTitle>Recent Calls</CardTitle>
                            <CardDescription>
                                Recent calls with their statuses.
                            </CardDescription>
                        </CardHeader>
                        <CardContent>
                            <Table>
                                <TableHeader>
                                    <TableRow>
                                        <TableHead>Contact</TableHead>
                                        <TableHead>Status</TableHead>
                                        <TableHead>Date</TableHead>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                    <TableRow v-for="(call, index) in recentCalls" :key="index">
                                        <TableCell class="py-6">{{ call.contact }}</TableCell>
                                        <TableCell>
                                            <Badge class="text-xs" :variant="call.status.variant">{{
                                                call.status.label
                                                }}</Badge>
                                        </TableCell>
                                        <TableCell>{{ call.date }}</TableCell>
                                    </TableRow>
                                </TableBody>
                            </Table>
                        </CardContent>
                    </Card>
                </TabsContent>
                <TabsContent value="upcoming-events">
                    <!-- Upcoming Events Section -->
                    <Card>
                        <CardHeader class="px-7">
                            <CardTitle>Upcoming Events</CardTitle>
                            <CardDescription>
                                Events scheduled for the upcoming week.
                            </CardDescription>
                        </CardHeader>
                        <CardContent>
                            <Table>
                                <TableHeader>
                                    <TableRow>
                                        <TableHead>Event</TableHead>
                                        <TableHead>Date</TableHead>
                                        <TableHead>Time</TableHead>
                                        <TableHead>Location</TableHead>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                    <TableRow v-for="(event, index) in upcomingEvents" :key="index">
                                        <TableCell class="py-6">{{ event.name }}</TableCell>
                                        <TableCell>{{ event.date }}</TableCell>
                                        <TableCell>{{ event.time }}</TableCell>
                                        <TableCell>{{ event.location }}</TableCell>
                                    </TableRow>
                                </TableBody>
                            </Table>
                        </CardContent>
                    </Card>
                </TabsContent>
                <TabsContent value="upcoming-tasks">
                    <!-- Upcoming Tasks Section -->
                    <Card>
                        <CardHeader class="px-7">
                            <CardTitle>Upcoming Tasks</CardTitle>
                            <CardDescription>
                                Tasks to be completed in the upcoming week.
                            </CardDescription>
                        </CardHeader>
                        <CardContent>
                            <Table>
                                <TableHeader>
                                    <TableRow>
                                        <TableHead>Task</TableHead>
                                        <TableHead>Due Date</TableHead>
                                        <TableHead>Status</TableHead>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                    <TableRow v-for="(task, index) in upcomingTasks" :key="index">
                                        <TableCell>{{ task.name }}</TableCell>
                                        <TableCell class="py-6">{{ task.dueDate }}</TableCell>
                                        <TableCell>
                                            <Badge class="text-xs" :variant="task.status.variant">{{ task.status.label
                                                }}</Badge>
                                        </TableCell>
                                    </TableRow>
                                </TableBody>
                            </Table>
                        </CardContent>
                    </Card>
                </TabsContent>
            </Tabs>



















        </div>
        <div class="flex flex-col gap-4">
            <Card>
                <CardHeader>
                    <CardTitle>Contact Support</CardTitle>
                    <CardDescription>
                        Need assistance? Contact our support team.
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <div />
                    <Button size="sm" class="gap-x-1.5" variant="secondary">
                        <Phone class="h-3 w-3"></Phone> Contact Support
                    </Button>
                </CardContent>
            </Card>
            <Card class="overflow-hidden">
                <CardHeader>
                    <CardTitle>Tutorials</CardTitle>
                    <CardDescription>
                        Explore tutorials to enhance your skills.
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <div class="grid gap-2">
                        <img alt="Tutorial image" class="aspect-square w-full rounded-md object-cover" height="300"
                            src="/placeholder.svg" width="300">
                        <div class="grid grid-cols-3 gap-2">
                            <button>
                                <img alt="Tutorial image" class="aspect-square w-full rounded-md object-cover"
                                    height="84" src="/placeholder.svg" width="84">
                            </button>
                            <button>
                                <img alt="Tutorial image" class="aspect-square w-full rounded-md object-cover"
                                    height="84" src="/placeholder.svg" width="84">
                            </button>
                            <button
                                class="flex aspect-square w-full items-center justify-center rounded-md border border-dashed">
                                <Upload class="h-4 w-4 text-muted-foreground" />
                                <span class="sr-only">Upload</span>
                            </button>
                        </div>
                    </div>
                </CardContent>
            </Card>


        </div>

    </main>

</template>